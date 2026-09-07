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

10_000.times do |i|
  data << Faker::Name.name
end

2.times { puts }
puts 'Benchmark: Compile SVG'

Benchmark.benchmark(Benchmark::Tms::CAPTION, 20) do |b|
  b.report('Compiling SVGs Processes') do
    Parallel.map(data, in_processes: 4) do |name|
      Typst(body: "= #{name}", concurrent: true).compile(:svg)
    end
  end

  b.report('Compiling SVGs Threads') do
    Parallel.map(data, in_threads: 4) do |name|
      Typst(body: "= #{name}", concurrent: true).compile(:svg)
    end
  end

  b.report('Compiling SVGs Single Thread (with GVL)') do
    data.each_with_index do |name, i|
      Typst(body: "= #{name}", concurrent: false).compile(:svg)
    end
  end
end

2.times { puts }
