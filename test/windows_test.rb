require "open3"
require "test/unit"
require_relative "../lib/typst"

# Windows keeps one hazard the other platforms do not have: libruby exports a
# symbol named `Sleep`, and it is `rb_w32_Sleep`, whose whole body is a
# `BLOCKING_REGION` - it releases and reacquires the GVL. Reached from a thread
# that has already released the GVL, which is every thread inside a compile, it
# corrupts Ruby's own bookkeeping and the next waiter dies with
# `[BUG] win32_mutex_lock: WAIT_ABANDONED`.
#
# `ext/typst/build.rs` keeps the extension away from it by linking with
# `--wrap=Sleep` and `--wrap=__imp_Sleep`, and `ext/typst/src/nogvl.rs` supplies
# both wraps. These two tests guard that arrangement from opposite sides: one
# reads the linked result, the other runs the workload that used to break.
class WindowsTest < Test::Unit::TestCase
  # The extension Ruby actually opened. `rake compile` leaves copies of it under
  # tmp/ as well, and a stale one is the classic way to test nothing at all.
  EXTENSION = $LOADED_FEATURES.grep(/typst\.so\z/).first

  def setup
    omit("Windows only") unless Gem.win_platform?
  end

  # A disassembly cannot answer this. The binding that `--wrap=__imp_Sleep`
  # closes was an indirect call through the import address table - mingw's
  # `_CRT_INIT` loads the slot into a register and calls through it - so a
  # search for a direct call to `Sleep` reports zero either way. The import
  # table says plainly which DLL supplies the symbol.
  def test_the_extension_imports_no_sleep_from_libruby
    dlls = PortableExecutable.new(EXTENSION).imports
    libruby = dlls.keys.grep(/ruby/i)

    assert_not_empty(libruby, "the extension imports nothing from libruby at all")

    assert_empty(libruby.select { |dll| dlls[dll].include?("Sleep") },
      "Sleep is binding to libruby again - see the wraps in ext/typst/build.rs")
  end

  # The other half of the same invariant: the wrap has to reach the real wait.
  # `SleepEx(ms, FALSE)` is `Sleep` without the alertable part, and libruby
  # exports no symbol by that name.
  def test_the_extension_imports_sleep_ex_from_kernel32
    dlls = PortableExecutable.new(EXTENSION).imports
    kernel32 = dlls.select { |dll, _| dll.casecmp?("kernel32.dll") }.values.flatten

    assert_include(kernel32, "SleepEx", "__wrap_Sleep is not reaching the Win32 call")
  end

  # What went wrong here was a `[BUG]` abort, which takes the whole process with
  # it - no assertion written inside it would ever run. So the workload goes in
  # a child, and this test reads what is left of it.
  #
  # The evictor barely pauses, because saturating comemo's write lock is what
  # drove parking_lot into its spin - and into `Sleep` - in the first place. It
  # does pause, though: with no pause at all the starvation factor is whatever
  # the lock feels like handing out, which on a two core runner is not bounded
  # by anything this test could wait for. A hundred microseconds still leaves
  # thousands of evictions a second to contend with.
  #
  # The child reports each document it finishes, and the count is what this
  # asserts. The timeout is then only there to turn a stall into a failure
  # rather than a hung suite - it does not police how long the work takes.
  def test_compiles_survive_an_unthrottled_evictor
    output, status, killed = ruby(<<~'CHILD', File.expand_path("../lib", __dir__), timeout: 180)
      require File.join(ARGV[0], "typst")

      source = <<~TYPST
        = Bericht

        #table(columns: 2, ..range(0, 200).map(i => [Zeile #i]))
      TYPST

      evicting = true
      evictor = Thread.new do
        while evicting
          Typst.clear_cache(0)
          sleep(0.0001)
        end
      end

      4.times.map do
        Thread.new do
          3.times do
            Typst(body: source, ignore_system_fonts: true).compile(:pdf)
            $stdout.puts("compiled")
            $stdout.flush
          end
        end
      end.each(&:join)

      evicting = false
      evictor.join
    CHILD

    assert_not_match(/\[BUG\]|WAIT_ABANDONED/, output,
      "a compile reached Ruby's Sleep with the GVL released")
    assert_false(killed, "the compiles stopped making progress and the child had to be killed")
    assert_predicate(status, :success?, "the child did not survive:\n#{output}")
    assert_equal(12, output.scan(/^compiled$/).count,
      "the child lost a document to the evictor:\n#{output}")
  end

  private

  # Runs `source` in a child interpreter with `arguments` in its ARGV, and
  # returns its merged output, its exit status, and whether it had to be killed.
  # A child that stops making progress is killed, so that the hang half of the
  # bug fails the test rather than stalling the suite behind it.
  #
  # That last return value is not redundant. Terminating a process on Windows
  # gives it exit status 0, so a killed child cannot be told apart from one that
  # ran to completion by its status alone.
  def ruby(source, *arguments, timeout:)
    Open3.popen2e(RbConfig.ruby, "-e", source, *arguments) do |input, output, child|
      input.close
      killed = false
      watchdog = Thread.new do
        next if child.join(timeout)

        killed = true
        Process.kill("KILL", child.pid)
      end

      text = output.read
      status = child.value
      watchdog.kill

      [text, status, killed]
    end
  end
end

# Just enough of the PE format to answer "what does this image import, and from
# which DLL". Field offsets are the ones in the PE specification, and every
# number in the format is little-endian.
class PortableExecutable
  def initialize(path)
    @image = File.binread(path)

    header = @image[0x3c, 4].unpack1("V")
    raise ArgumentError, "#{path}: not a PE image" unless @image[header, 4] == "PE\0\0"

    sections = @image[header + 6, 2].unpack1("v")
    optional_size = @image[header + 20, 2].unpack1("v")
    optional = header + 24
    raise ArgumentError, "#{path}: not a 64 bit image" unless @image[optional, 2].unpack1("v") == 0x20b

    @sections = Array.new(sections) do |i|
      entry = optional + optional_size + i * 40
      size, address, raw_size, raw = @image[entry + 8, 16].unpack("V4")

      # A section can be longer in memory than on disk, and shorter: the file
      # rounds up to a disk block, memory rounds up to a page.
      { address: address, size: [size, raw_size].max, raw: raw }
    end

    # The data directory follows the 112 byte PE32+ optional header, and its
    # second entry is the import table.
    @import_table = @image[optional + 112 + 8, 4].unpack1("V")
  end

  # DLL name => the function names imported from it.
  def imports
    dlls = Hash.new { |imported, dll| imported[dll] = [] }
    descriptor = offset_of(@import_table)

    loop do
      lookup, _timestamp, _forwarder, name, addresses = @image[descriptor, 20].unpack("V5")
      break if lookup.zero? && name.zero? && addresses.zero?

      # Both tables list the same imports. The lookup table is the one that
      # keeps their names; the address table is overwritten with the resolved
      # addresses when the image is bound, and is missing from some linkers.
      thunk = offset_of(lookup.zero? ? addresses : lookup)
      dll = string_at(name)

      until (entry = @image[thunk, 8].unpack1("Q<")).zero?
        # The top bit marks an import by ordinal, which carries no name. The
        # rest is the address of a two byte hint followed by the name.
        dlls[dll] << string_at((entry & 0x7fff_ffff) + 2) if entry[63].zero?
        thunk += 8
      end

      descriptor += 20
    end

    dlls
  end

  private

  def string_at(address)
    @image[offset_of(address)..].unpack1("Z*")
  end

  # Addresses in the image are relative to where it would be loaded. The file
  # lays the same bytes out differently, section by section.
  def offset_of(address)
    section = @sections.find { |s| (s[:address]...s[:address] + s[:size]).cover?(address) }
    raise ArgumentError, "address 0x#{address.to_s(16)} falls in no section" unless section

    address - section[:address] + section[:raw]
  end
end
