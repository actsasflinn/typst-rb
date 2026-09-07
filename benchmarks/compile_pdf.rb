require 'benchmark'
require 'rubygems'
require 'faker'

require_relative "../lib/typst"

data = []

10_000.times do |i|
  data << Faker::Name.name
end

2.times { puts }
puts 'Compile PDF'

Benchmark.benchmark(' ' * 20 + Benchmark::Tms::CAPTION, 20) do |b|
  b.report('Compiling PDFs Sync') do
    data.each_with_index do |name, i|
      Typst(body: "= #{name}").compile(:pdf)
    end
  end
end
