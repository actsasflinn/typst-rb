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

# Compile a template file and write the output to a PDF file
Typst("readme.typ").compile(:pdf).write("readme.pdf")

# Use a typst template file `readme.typ`
t = Typst("readme.typ")

# Use a typst template string
t = Typst(template: %{hello world})

# Use a typst template in a zip file
t = Typst(zip: "test/main.typ.zip")

# Compile to PDF
f = t.compile(:pdf)

# Compile to SVG
f = t.compile(:svg)

# Compile to PNG
f = t.compile(:png)

# Compile to SVGs enveloped in HTML
# Depracation warning: this feature will go away once Typst HTML moves out of experimental
f = t.compile(:html, title: "Typst+Ruby")

# Compile to HTML (using Typst expirmental HTML)
f = t.compile(:html_experimental)

# Access PDF or HTML output as a string
# Note: For PDF and PNG this will give data, for SVG and HTML this will give markup
Typst("readme.typ").compile(:pdf).document
# => "%PDF-1.7\n%\x80\x80\x80\x80\n\n4 0 obj\n<<\n  /Type /Font\n  /Subtype ..." 
Typst("readme.typ").compile(:html).document
# => "\n<!DOCTYPE html>\n<html>\n<head>\n<title>main</title>\n</head>\n<body>\n<svg class=\"typst-doc\" ...

# Or return content as an array of bytes
pdf_bytes = Typst("readme.typ").compile(:pdf).bytes
# => [37, 80, 68, 70, 45, 49, 46, 55, 10, 37, 128 ...]

# Write the output to a file
# Note: for multi-page documents using formats other than PDF, pages write to multiple files, e.g. `readme_0.png`, `readme_1.png`
f.write("filename.pdf")

# Return SVG, HTML or PNG content as an array of pages
Typst("readme.typ").compile(:svg).pages
# => ["<svg class=\"typst-doc\" viewBox=\"0 0 595.2764999999999 841.89105\" ..."
Typst("readme.typ").compile(:html).pages
# => ["<svg class=\"typst-doc\" viewBox=\"0 0 595.2764999999999 841.89105\" ..."
Typst("readme.typ").compile(:png).pages
# => ["\x89PNG\r\n\x1A\n\x00\x00\x00\rIHDR\x00\x00\x04\xA7\x00\x00\x06\x94\b\ ...

# Pass values into a typst template using Typst sys_inputs
sys_inputs_example = %{
#let persons = json(bytes(sys.inputs.persons))

#for person in persons [
  #person.name is #person.age years old.\\
]
}
people = [{"name" => "John", "age" => 35}, {"name" => "Xoliswa", "age" => 45}]
data = { "persons" => people.to_json }
Typst(template: sys_inputs_example, sys_inputs: data).compile(:pdf).write("sys_inputs_example.pdf")

# Apply inputs to a template to product multiple PDFs

t = Typst(template: sys_inputs_example)
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

Typst(template: main, dependencies: { "template.typ" => template, "icon.svg" => icon }, fonts: { "Example.ttf" => font_bytes }).compile(:pdf)
 
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

```

## Contributors & Acknowledgements
typst-rb is based on [typst-py](https://github.com/messense/typst-py) by [messense](https://github.com/messense)

## License

This work is released under the Apache-2.0 license. A copy of the license is provided in the [LICENSE](./LICENSE) file.
