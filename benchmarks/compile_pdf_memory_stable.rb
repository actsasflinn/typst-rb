require 'bundler/inline'

gemfile do
  source 'https://rubygems.org'
  gem 'benchmark'
  gem 'faker'
  gem 'parallel'
  gem 'rubyzip', "~> 3.2"
  gem 'memory_profiler'
#  gem 'typst', "0.15.1.9.pre"
  gem 'typst', "0.15.1.8"
end

require 'benchmark'
require 'rubygems'
require 'faker'
require 'parallel'
require 'memory_profiler'

require "typst"

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

font_bytes = File.read("test/fonts/Fasthand/Release/ttf/Fasthand-Regular.ttf")
t = Typst(body: main, fonts: { "Fasthand-Regular.ttf" => font_bytes }, concurrent: true)

2.times { puts }
puts "Memory #{data.size}: Compile PDF (typst-rb 0.15.1.8)"

report = MemoryProfiler.report do
  data.each do |person|
    t.with_inputs({ "persons" => person }).compile(:pdf)
  end
end
report.pretty_print(scale_bytes: true)
