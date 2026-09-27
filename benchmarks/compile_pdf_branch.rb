require 'bundler/inline'

gemfile do
  source 'https://rubygems.org'
  gem 'benchmark'
  gem 'faker'
  gem 'parallel'
  gem 'rubyzip', "~> 3.2"
end

require 'benchmark'
require 'rubygems'
require 'faker'
require 'parallel'

require_relative "../lib/typst"

data = []

10000.times do |i|
  data << [{"name" => Faker::Name.name, "age" => rand(85)}].to_json
end

main = %{
#set text(12pt, font: "Fasthand")
#let persons = json(bytes(sys.inputs.persons))

#for person in persons [
  #person.name is #person.age years old.\\
]
}

font = File.read("test/fonts/Fasthand/Release/ttf/Fasthand-Regular.ttf")
Typst::add_font(font.bytes)
t = Typst::VirtualWorld.new(main)

2.times { puts }
puts "Benchmark #{data.size}: Compile PDF (typst-rb feat/in-memory branch)"

Benchmark.benchmark(Benchmark::Tms::CAPTION, 20) do |b|
  b.report('Compiling PDFs Processes') do
    Parallel.map(data, in_processes: 4) do |person|
      t.with_inputs({ "persons" => person }).to_pdf().inspect
    end
  end

  b.report('Compiling PDFs Threads') do
    Parallel.map(data, in_threads: 4) do |person|
      t.with_inputs({ "persons" => person }).to_pdf().inspect
    end
  end

  b.report('Compiling PDFs Single Thread (with GVL)') do
    data.each do |person|
      t.with_inputs({ "persons" => person }).to_pdf().inspect
    end
  end
end