require "test/unit"
require_relative "../lib/typst"

# Compiling releases the GVL (see ext/typst/src/nogvl.rs) when asked to, so
# several Ruby threads can sit inside the compiler at the same time. Everything
# the compiler reaches for is process-global - comemo's memoization cache, the
# package storage, font discovery - so these tests pin down what concurrent
# compiles are allowed to do to each other.
#
# Releasing is opt-in, so the suite turns it on for itself.
#
# HTML is the export format of choice here. It comes back as plain text, so a
# marker passed in through sys_inputs can be read straight out of the result,
# and unlike a PDF it carries no timestamp that would make two compiles of the
# same source differ.
class ConcurrencyTest < Test::Unit::TestCase
  THREADS = 8

  def setup
    @release_gvl = Typst.release_gvl
    Typst.release_gvl = true
  end

  def teardown
    Typst.release_gvl = @release_gvl
  end

  def in_threads(count = THREADS, &block)
    count.times.map { |i| Thread.new { block.call(i) } }.map(&:value)
  end

  def html(body, **options)
    Typst(body: body, **options).compile(:html_experimental)
  end

  # A table gives the compiler enough to do that the threads genuinely overlap
  # rather than finishing one after another by accident.
  def report(marker)
    %{= #{marker}\n\n#table(columns: 2, ..range(0, 200).map(i => [Zeile #i]))}
  end

  # The premise of everything below, and the one thing nothing else here would
  # notice going away: a compile releases the GVL. A serial implementation
  # produces the same documents, the same inputs, the same warnings and the same
  # errors, so every other test in this file passes with the release removed.
  #
  # An extension holding the GVL never yields to Ruby, so the ticker gets no
  # turns at all in that case and a great many in this one. That makes it an
  # ordering fact rather than a duration - no wall clock, and no dependence on
  # how many cores happen to be around.
  #
  # Two details keep it that way. `Typst::_to_pdf` is called directly, because
  # everything the wrapper does around it - the temporary directory, the file
  # writes - releases the GVL by itself and would tick either way. And the
  # measured compile is preceded by a warm-up and by handing the ticker the GVL,
  # so that it starts on a fresh timeslice and stays well inside it: a compile
  # long enough to be preempted would be scheduled out on return and could tick
  # after the fact.
  def ticks_during_compile(release_gvl)
    Dir.mktmpdir do |directory|
      file = Pathname.new(directory).join("main.typ")
      File.write(file, report("Bericht"))
      arguments = Typst(file: file.to_s, root: directory, ignore_system_fonts: true, release_gvl: release_gvl).typst_pdf_args
      assert_equal(release_gvl, arguments.last)
      Typst::_to_pdf(*arguments)

      ticks = 0
      running = true
      ticker = Thread.new { ticks += 1 while running }

      begin
        Thread.pass
        ticks = 0
        Typst::_to_pdf(*arguments)
        ticks
      ensure
        running = false
        ticker.join
      end
    end
  end

  def test_compiling_lets_other_ruby_threads_run
    assert_operator(ticks_during_compile(true), :>, 0, "the compile held the GVL for its whole duration")
  end

  # The other side of the switch, and the reason it exists: without release_gvl the
  # extension behaves as it always did and nothing else gets a turn.
  def test_compiling_holds_the_gvl_by_default
    assert_equal(0, ticks_during_compile(false), "the compile released the GVL without being asked to")
  end

  def test_release_gvl_defaults_to_the_global_setting
    Typst.release_gvl = false
    assert_equal(false, Typst(body: "= Bericht").options[:release_gvl])
    assert_equal(true, Typst(body: "= Bericht", release_gvl: true).options[:release_gvl])

    Typst.release_gvl = true
    assert_equal(true, Typst(body: "= Bericht").options[:release_gvl])
    assert_equal(false, Typst(body: "= Bericht", release_gvl: false).options[:release_gvl])
  end

  def test_concurrent_compiles_of_one_input_match_a_serial_compile
    source = report("Bericht")
    expected = html(source).document

    in_threads { html(source).document }.each_with_index do |document, i|
      assert_equal(expected, document, "thread #{i} produced a different document")
    end
  end

  # comemo memoizes on a hash of the world, the standard library included, and
  # sys.inputs is part of that library. Two threads compiling the same source
  # with different inputs have to miss each other's cache entries.
  def test_concurrent_compiles_keep_their_own_sys_inputs
    markers = THREADS.times.map { |i| "marker-#{i}-#{rand(1 << 32)}" }
    source = %{#sys.inputs.marker}

    documents = in_threads do |i|
      html(source, sys_inputs: { "marker" => markers[i] }).document
    end

    documents.each_with_index do |document, i|
      assert_include(document, markers[i])
      (markers - [markers[i]]).each do |other|
        assert_not_include(document, other, "thread #{i} picked up another thread's input")
      end
    end
  end

  def test_concurrent_compiles_of_different_documents_do_not_mix
    markers = THREADS.times.map { |i| "Dokument-#{i}" }

    documents = in_threads { |i| html(report(markers[i])).document }

    documents.each_with_index do |document, i|
      assert_include(document, markers[i])
      (markers - [markers[i]]).each do |other|
        assert_not_include(document, other, "thread #{i} picked up another thread's source")
      end
    end
  end

  # Warnings are collected per compile and travel out of the GVL-free region as
  # a plain Vec<String>, so a warning must not surface on a thread that did not
  # earn it. Odd-numbered threads name a font nobody has. PDF rather than HTML
  # here, because HTML export warns about its own experimental status on every
  # single compile and would drown the signal.
  def test_warnings_stay_with_their_own_compile
    documents = in_threads do |i|
      font = i.odd? ? %{#set text(font: "No Such Font Family ZZZ")\n} : ""
      Typst(body: font + report("Heading #{i}")).compile(:pdf)
    end

    documents.each_with_index do |document, i|
      if i.odd?
        assert(document.warnings?, "thread #{i} lost its warning")
        assert(document.warnings.any? { |w| w.include?("unknown font family") })
      else
        assert_equal([], document.warnings, "thread #{i} was handed another thread's warning")
      end
    end
  end

  # Errors leave the GVL-free region as plain strings and are turned into
  # exceptions once the GVL is back. Each thread has to receive its own.
  def test_concurrent_failures_raise_their_own_error
    errors = in_threads do |i|
      if i.odd?
        assert_raise(ArgumentError) { html(%{#let fehler#{i} = }) }
      else
        html(report("ok")).document
        nil
      end
    end

    errors.each_with_index do |error, i|
      next if i.even?
      assert_include(error.message, "expected expression")
      assert_include(error.message, "fehler#{i}")
    end
  end

  # Typst.clear_cache still runs while holding the GVL, so it now races the
  # compiles in other threads for comemo's write lock. Evicting mid compile must
  # neither deadlock nor cost a document.
  #
  # The evictor is throttled on purpose. An unthrottled clear_cache(0) loop
  # takes the write lock hundreds of thousands of times a second and starves the
  # compiling threads by a factor of 400 - that is a documented sharp edge, not
  # something to reproduce on every test run.
  def test_clear_cache_during_concurrent_compiles
    documents = 0
    evictions = 0
    counter = Mutex.new
    done = false

    workers = 4.times.map do |i|
      Thread.new do
        10.times do
          document = Typst(body: report("Doc #{i}"), ignore_system_fonts: true).compile(:pdf).document
          assert(document.start_with?("%PDF"))
          counter.synchronize { documents += 1 }
        end
      end
    end

    evictor = Thread.new do
      until done
        Typst.clear_cache(0)
        evictions += 1
        sleep(0.001)
      end
    end

    begin
      workers.each(&:join)
    ensure
      done = true
      evictor.join
    end

    assert_equal(40, documents)
    assert_operator(evictions, :>, 0, "the evictor never ran, so nothing was raced")
  end

  # Typst.clear_font_cache drops the shared font stores while other threads are
  # compiling against them. An in-flight compile holds its own Arc, so it must
  # finish against the store it started with; the next one rebuilds.
  #
  # The clears are throttled the same way, because an unthrottled loop means
  # every compile rescans the system font directories and the test takes
  # minutes. ignore_system_fonts is off here on purpose: the scan is the part
  # being invalidated.
  def test_clear_font_cache_during_concurrent_compiles
    documents = 0
    clears = 0
    counter = Mutex.new
    done = false

    workers = 4.times.map do |i|
      Thread.new do
        5.times do
          document = Typst(body: report("Doc #{i}")).compile(:pdf).document
          assert(document.start_with?("%PDF"))
          counter.synchronize { documents += 1 }
        end
      end
    end

    clearer = Thread.new do
      until done
        Typst.clear_font_cache
        clears += 1
        sleep(0.05)
      end
    end

    begin
      workers.each(&:join)
    ensure
      done = true
      clearer.join
    end

    assert_equal(20, documents)
    assert_operator(clears, :>, 0, "the cache was never cleared, so nothing was raced")
  end

  # Package storage is shared across threads, and what this test covers is the
  # warm cache: concurrent reads of a package already on disk, which is the
  # common case. The serial compile below is what makes it warm - on a fresh
  # machine, CI included, the cache starts out empty and the fan-out would race
  # the download instead.
  #
  # It does not cover a cold one, and that gap is deliberate rather than
  # incidental. `prepare_package` in ext/typst/src/package.rs untars a download
  # straight into its final directory - no temporary directory, no rename into
  # place - and removes that directory again if the archive turns out to be
  # malformed. Two threads obtaining the same missing package therefore write
  # over each other, and on Windows they do it harder than elsewhere, because
  # creating a file another thread still holds open fails outright rather than
  # clobbering. Reaching that state from a test means deleting the real package
  # cache: `dirs::cache_dir()` goes through the known-folder API on Windows, so
  # no environment variable can point it somewhere disposable.
  def test_concurrent_package_obtain
    source = %{#import "@preview/invoice-maker:1.1.0": *\n= Paket da}
    html(source).document

    in_threads(4) { html(source).document }.each_with_index do |document, i|
      assert_include(document, "Paket da", "thread #{i} did not get the package")
    end
  end
end
