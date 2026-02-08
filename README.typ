#set document(title: [typst.rb README])
#show link: underline
#show link: set text(blue)
#show raw.where(block: true): block.with(
  fill: luma(240),
  inset: 10pt,
  radius: 4pt,
)

= typst-rb

Ruby binding to #link("https://github.com/typst/typst")[typst], a new markup-based typesetting system that is powerful and easy to learn.

== Installation

```bash
gem install typst
```

== Usage

```ruby
require "typst"

# Compile a typst file and write the output to a PDF file
Typst("readme.typ").compile(:pdf).write("readme.pdf")
```

=== Use a typst file `readme.typ`
```ruby
t = Typst("readme.typ")
```

=== Use a typst string
```ruby
t = Typst(body: %{hello world})
```

=== Use a typst file in a zip file
```ruby
t = Typst(zip: "test/main.typ.zip")
```

=== Compile to PDF
```ruby
doc = t.compile(:pdf)
```

=== Compile to PDF selecting the typst supported PdfStandard
```ruby
doc = t.compile(:pdf, pdf_standards: ["2.0"])
```

=== Compile to SVG
```ruby
doc = t.compile(:svg)
```

=== Compile to PNG
```ruby
doc = t.compile(:png)
```

=== Compile to PNG and set PPI
```ruby
doc = t.compile(:png, ppi: 72)
```

=== Compile to HTML (using Typst expirmental HTML)
```ruby
doc = t.compile(:html_experimental)
```

=== Or return content as an array of bytes
```ruby
pdf_bytes = Typst("readme.typ").compile(:pdf).bytes
# => [37, 80, 68, 70, 45, 49, 46, 55, 10, 37, 128 ...]
```

=== Write the output to a file
Note: for multi-page documents using formats other than PDF and HTML, pages write to multiple files, e.g. `readme_0.png`, `readme_1.png`
```ruby
doc.write("filename.pdf")
```

=== Return PDF, SVG, PNG or HTML content as an array of pages
```ruby
Typst("readme.typ").compile(:pdf).pages
# => ["%PDF-1.7\n%\x80\x80\x80\x80\n\n1 0 obj\n<<\n  /Type /Pages\n  /Count 3\n  /Kids [160 0 R 162 ...

Typst("readme.typ").compile(:svg).pages
# => ["<svg class=\"typst-doc\" viewBox=\"0 0 595.2764999999999 841.89105\" ...

Typst("readme.typ").compile(:png).pages
# => ["\x89PNG\r\n\x1A\n\x00\x00\x00\rIHDR\x00\x00\x04\xA7\x00\x00\x06\x94\b\ ...

Typst("readme.typ").compile(:html_experimental).pages
# => ["<!DOCTYPE html>\n<html>\n  <head>\n    <meta charset=\"utf-8\">\n    <meta name=\"viewport\" ...
```

=== Pass values into typst using sys_inputs
```ruby
sys_inputs_example = %{
#let persons = json(bytes(sys.inputs.persons))

#for person in persons [
  #person.name is #person.age years old.\\
]
}
people = [{"name" => "John", "age" => 35}, {"name" => "Xoliswa", "age" => 45}]
data = { "persons" => people.to_json }
Typst(body: sys_inputs_example, sys_inputs: data).compile(:pdf).write("sys_inputs_example.pdf")
```

=== Apply inputs to typst to product multiple PDFs
```ruby
t = Typst(body: sys_inputs_example)
people.each do |person|
  t.with_inputs({ "persons" => [person].to_json }).compile(:pdf).write("#{person['name']}.pdf")
end
```

=== A more complex example of compiling from string using other dependency typst template, svg and font resources all in memory
```ruby
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
```
 
=== From a zip with a named main typst file
```ruby
Typst(zip: "test/main.typ.zip", main_file: "hello.typ").compile(:pdf)
```

=== Use a package from the #link("https://typst.app/universe")[Typst Universe]
Your package_example.typ file...
```typst
#import "@preview/wordometer:0.1.5": word-count, total-words
#show: word-count

In this document, there are #total-words words all up.

#word-count(total => [
  The number of words in this block is #total.words
  and there are #total.characters letters.
])
```
...compiles just like anything else
```ruby
Typst("package_example.typ").compile(:pdf).write("package_example.pdf")
```

=== Query a typst document
```ruby
Typst("readme.typ").query("heading").result
# => 
# [{"func" => "heading",
#   "level" => 1,
#   "depth" => 1,
# ...

Typst("readme.typ").query("heading", format: "json").result(raw: true)
# => "[\n  {\n    \"func\": \"heading\",\n    \"level\": 1,\n    \"depth\": ..."

Typst("readme.typ").query("heading", format: "yaml").result(raw: true)
# => "- func: heading\n  level: 1\n  depth: 1\n  offset: 0\n  numbering: ..."

# Query results as JSON string
Typst("test/test.typ").query("heading").to_s
# => "[\n  {\n    \"func\": \"heading\",\n    \"level\": 1,\n    \"depth\": 1,\n    \"offset\": 0,\n ...

# Query results as YAML string
Typst("test/test.typ").query("heading", format: "yaml").to_s
# => "- func: heading\n  level: 1\n  depth: 1\n  offset: 0\n  numbering: null\n  supplement:\n    ...
```

=== clear the compilation cache
```ruby
# Evict all entries whose age is larger than or equal to `max_age`
max_age = 10
Typst::clear_cache(max_age)
```

== Contributors & Acknowledgements
typst-rb is based on #link("https://github.com/messense/typst-py")[typst-py] by #link("https://github.com/messense")[messense]\
clear_cache was contributed by #link("https://github.com/NRicciVestmark")[NRicciVestmark]

== License

This work is released under the Apache-2.0 license. A copy of the license is provided in the #link("https://github.com/actsasflinn/typst-rb/blob/main/LICENSE")[LICENSE] file.