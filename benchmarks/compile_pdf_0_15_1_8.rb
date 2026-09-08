require 'bundler/inline'

gemfile do
  source 'https://rubygems.org'
  gem 'benchmark'
  gem 'faker'
  gem 'parallel'
  gem 'rubyzip', "~> 3.2"
  gem 'typst', "= 0.15.1.8"
end

require 'benchmark'
require 'rubygems'
require 'faker'
require 'parallel'
require 'typst'

data = []

10_000.times do |i|
  data << {"name" => Faker::Name.name, "age" => rand(85)}
end

main = %{
#let persons = json(bytes(sys.inputs.persons))

#for person in persons [
  #person.name is #person.age years old.\\
]
}

t = Typst(body: main, concurrent: true)

2.times { puts }
puts "Benchmark #{data.size}: Compile PDF (typst-rb 0.15.1.8 with font optimization)"

Benchmark.benchmark(Benchmark::Tms::CAPTION, 20) do |b|
  b.report('Compiling PDFs Processes') do
    Parallel.map(data, in_processes: 4) do |person|
      t.with_inputs({ "persons" => [person].to_json }).compile(:pdf)
    end
  end

  b.report('Compiling PDFs Threads') do
    Parallel.map(data, in_threads: 4) do |person|
      t.with_inputs({ "persons" => [person].to_json }).compile(:pdf)
    end
  end

  b.report('Compiling PDFs Single Thread (with GVL)') do
    data.each do |person|
      t.with_inputs({ "persons" => [person].to_json }).compile(:pdf)
    end
  end
end
