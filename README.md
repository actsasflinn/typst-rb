# typst-rb

Ruby binding to [typst](https://github.com/typst/typst),
a new markup-based typesetting system that is powerful and easy to learn.

## Installation

```bash
gem install typst
```

## Usage

```ruby
require "typst"

# Compile a typst file and write the output to a PDF file
Typst("readme.typ").compile(:pdf).write("readme.pdf")

# Use a typst file `readme.typ`
t = Typst("readme.typ")

# Use a typst string
t = Typst(body: %{hello world})

# Use a typst file in a zip file
t = Typst(zip: "test/main.typ.zip")

# Compile to PDF
doc = t.compile(:pdf)

# Compile to SVG
doc = t.compile(:svg)

# Compile to PNG
doc = t.compile(:png)

# Compile to HTML (using Typst expirmental HTML)
doc = t.compile(:html_experimental)

# Or return content as an array of bytes
pdf_bytes = Typst("readme.typ").compile(:pdf).bytes
# => [37, 80, 68, 70, 45, 49, 46, 55, 10, 37, 128 ...]

# Write the output to a file
# Note: for multi-page documents using formats other than PDF and HTML, pages write to multiple files, e.g. `readme_0.png`, `readme_1.png`
doc.write("filename.pdf")

# Return PDF, SVG, PNG or HTML content as an array of pages
Typst("readme.typ").compile(:pdf).pages
# => ["%PDF-1.7\n%\x80\x80\x80\x80\n\n1 0 obj\n<<\n  /Type /Pages\n  /Count 3\n  /Kids [160 0 R 162 ...
Typst("readme.typ").compile(:svg).pages
# => ["<svg class=\"typst-doc\" viewBox=\"0 0 595.2764999999999 841.89105\" ...
Typst("readme.typ").compile(:png).pages
# => ["\x89PNG\r\n\x1A\n\x00\x00\x00\rIHDR\x00\x00\x04\xA7\x00\x00\x06\x94\b\ ...
Typst("readme.typ").compile(:html_experimental).pages
# => ["<!DOCTYPE html>\n<html>\n  <head>\n    <meta charset=\"utf-8\">\n    <meta name=\"viewport\" ...

# Pass values into typst using sys_inputs
sys_inputs_example = %{
#let persons = json(bytes(sys.inputs.persons))

#for person in persons [
  #person.name is #person.age years old.\\
]
}
people = [{"name" => "John", "age" => 35}, {"name" => "Xoliswa", "age" => 45}]
data = { "persons" => people.to_json }
Typst(body: sys_inputs_example, sys_inputs: data).compile(:pdf).write("sys_inputs_example.pdf")

# Apply inputs to typst to product multiple PDFs

t = Typst(body: sys_inputs_example)
people.each do |person|
  t.with_inputs({ "persons" => [person].to_json }).compile(:pdf).write("#{person['name']}.pdf")
end

# A more complex example of compiling from string
main = %{
#import "template.typ": *

#show: template.with()

#lorem(50)

#image("icon.svg")
}

template = %{
#let template(body) = {
  set text(12pt, font: "Example")
  body
}
}

icon = File.read("icon.svg")
font_bytes = File.read("Example.ttf")

Typst(body: main, dependencies: { "template.typ" => template, "icon.svg" => icon }, fonts: { "Example.ttf" => font_bytes }).compile(:pdf)
 
# From a zip with a named main typst file
Typst(zip: "test/main.typ.zip", main_file: "hello.typ").compile(:pdf)

Typst::Query.new("heading", "readme.typ").result
# => 
# [{"func" => "heading",
#   "level" => 1,
#   "depth" => 1,
# ...

Typst::Query.new("heading", "readme.typ", format: "json").result(raw: true)
# => "[\n  {\n    \"func\": \"heading\",\n    \"level\": 1,\n    \"depth\": ..."

Typst::Query.new("heading", "readme.typ", format: "yaml").result(raw: true)
# => "- func: heading\n  level: 1\n  depth: 1\n  offset: 0\n  numbering: ..."

# clear the compilation cache
# Evict all entries whose age is larger than or equal to `max_age`
max_age = 10
Typst::clear_cache(max_age)
```

## Contributors & Acknowledgements
typst-rb is based on [typst-py](https://github.com/messense/typst-py) by [messense](https://github.com/messense)\
clear_cache was contributed by [NRicciVestmark](https://github.com/NRicciVestmark)

## License

This work is released under the Apache-2.0 license. A copy of the license is provided in the [LICENSE](./LICENSE) file.
