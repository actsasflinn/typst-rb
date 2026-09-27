require 'bundler/inline'

gemfile do
  source 'https://rubygems.org'
  gem 'benchmark'
  gem 'faker'
  gem 'parallel'
  gem 'rubyzip', "~> 3.2"
  gem 'memory_profiler'
end

require 'benchmark'
require 'rubygems'
require 'faker'
require 'parallel'
require 'memory_profiler'

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
puts "Memory #{data.size}: Compile PDF (typst-rb feat/in-memory branch)"

report = MemoryProfiler.report do
  data.each do |person|
    t.with_inputs({ "persons" => person }).to_pdf()
  end
end
report.pretty_print(scale_bytes: true)
