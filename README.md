# typst-rb

Ruby language binding to [typst](https://github.com/typst/typst),
a new markup-based typesetting system that is powerful and easy to learn.

## Rubygems

 Source and native gems are provided for the following platforms: `aarch64-linux` `aarch64-linux-musl` `arm64-darwin` `x64-mingw-ucrt` `x86_64-darwin` `x86_64-linux` `x86_64-linux-musl`. The gems are built from CI [gem-push](https://github.com/actsasflinn/typst-rb/actions/workflows/gem-push.yml) action and can be verified against SHA256SUMS published with the [release](https://github.com/actsasflinn/typst-rb/releases).

## Installation

Add the following to your gemfile and `bundle install`
```ruby
gem 'typst', '>= 0.15.1.5'
```
or install from the command line:
```bash
gem install typst
```

## Usage

```ruby
require "typst"
```

### Hello World
This example compiles a typst string to PDF and writes to a file.
```ruby
Typst(body: "= Hello World\nThis is your first typst PDF").compile(:pdf).write("hello_world.pdf")
```

### The basics
This example initializes a `Typst::Base` object `t` using a string. The `t` object is basically an environment for your typst input and can take a variety of options suitable for your task which you can see in subsequent examples.
```ruby
t = Typst(body: "= Hello World\nThis is your first typst PDF")
```
This step compiles the typst input held in the `t` object and returns a `Typst::Document` object `doc`. In this case we're compiling to PDF so `doc` is a `Typst::PdfDocument` object which holds the PDF bytes and is ready to write to a file, output to a buffer, etc.
```ruby
doc = t.compile(:pdf)
```
This step writes the output to a local file in your current working directory.
```ruby
doc.write("hello_world.pdf")
```
### Different ways to setup the typst input

#### Use a local typst file named `example.typ`
```ruby
t = Typst("example.typ")
```

#### Use a typst string
```ruby
t = Typst(body: %{hello world})
```

#### Use a zipped typst file
```ruby
t = Typst(zip: "test/main.typ.zip")
```

#### Use a remote typst file
```ruby
require "open-uri"
URI.open("https://github.com/actsasflinn/typst-rb/raw/refs/heads/main/README.typ") do |u|
  Typst(body: u.read).compile(:pdf).write("remote_readme.pdf")
end
```

#### Use a remote zipped typst file
```ruby
require "open-uri"
URI.open("https://github.com/actsasflinn/typst-rb/raw/refs/heads/main/test/hello.typ.zip") do |u|
  Tempfile.create do |f|
    f.write(u.read)
    f.rewind
    Typst(zip: f).compile(:pdf).write("remote_zipped.pdf")
  end
end
```

### Compiling

#### Compile to PDF
```ruby
doc = t.compile(:pdf)
```

#### Compile to PDF selecting the typst supported PdfStandard
```ruby
doc = t.compile(:pdf, pdf_standards: ["2.0"])
```

#### Compile to SVG
```ruby
doc = t.compile(:svg)
```

#### Compile to PNG
```ruby
doc = t.compile(:png)
```

#### Compile to PNG and set PPI
```ruby
doc = t.compile(:png, ppi: 72)
```

#### Compile to HTML (using Typst expirmental HTML)
```ruby
doc = t.compile(:html_experimental)
```

### Output

#### Return compiled content as an array of bytes
```ruby
pdf_bytes = Typst("readme.typ").compile(:pdf).bytes
# => [37, 80, 68, 70, 45, 49, 46, 55, 10, 37, 128 ...]
```

#### Write compiled output to a file
Note: for multi-page documents using formats other than PDF and HTML, pages write to multiple files, e.g. `filename_0.png`, `filename_1.png`
```ruby
doc.write("filename.pdf")
```

#### Return PDF, SVG, PNG or HTML content as an array of pages
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

### More advanced setups

#### Pass values into typst using sys_inputs
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

#### Apply inputs to typst to product multiple PDFs
```ruby
t = Typst(body: sys_inputs_example)
people.each do |person|
  t.with_inputs({ "persons" => [person].to_json }).compile(:pdf).write("#{person['name']}.pdf")
end
```

#### A more complex example of compiling from string using other dependency typst template, svg and font resources all in memory
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
 
#### Use a zip file with an alternatively named main typst file
```ruby
Typst(zip: "test/main.typ.zip", main_file: "hello.typ").compile(:pdf)
```

#### Use a package from the [Typst Universe](https://typst.app/universe)
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

### Compiler warnings

A compile can succeed *and* warn. The most common case is an unknown font
family: typst substitutes a different face, the document is produced, and
nothing tells you it is in the wrong typeface. Every compiled document carries
the warnings that came with it.

```ruby
doc = Typst(body: %{#set text(font: "Not Installed")\n= Hello}).compile(:pdf)
doc.warnings?
# => true
doc.warnings
# => ["warning: unknown font family: \"Not Installed\"\n  ┌─ /tmp/.../main.typ:1:0\n ..."]
```

Each entry is one formatted diagnostic, so they can be counted, filtered or
logged individually. A clean compile returns an empty array. Warnings that
accompany a compile *error* are still included in the raised message, as before.

### Query a typst document
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

### Threads and concurrency

Compiling releases Ruby's global VM lock, so several threads can compile at the
same time and genuinely use several cores.

```ruby
docs = invoices.map { |invoice| Thread.new { Typst(body: invoice).compile(:pdf) } }.map(&:value)
```

Eight threads compiling the same document, measured on a 24-core machine:

| threads | documents/second | cores busy |
|--------:|-----------------:|-----------:|
|       1 |            1062 |       1.36 |
|       8 |            6613 |      10.17 |

That is throughput across documents, not speed within one. typst parallelizes
page runs, so a document with a single page layout is one run and gains almost
nothing from the threads it is given.

Threads share the compilation cache and the discovered fonts. Compiling
different sources, with different `sys_inputs`, font paths and roots, at the
same time is safe.

Two things to know:

* A compile cannot be interrupted. `Thread#kill` and `Ctrl-C` take effect once
  it returns.
* Package storage is not safe to populate concurrently. Threads that need the
  same not-yet-cached package each download their own copy and untar it
  straight into the same directory, over each other; on Windows that fails
  outright rather than clobbering. Compile once to warm the cache before
  fanning out. Reading a package already on disk from many threads is safe.

### clear the compilation cache
```ruby
# Evict all entries whose age is larger than or equal to `max_age`
max_age = 10
Typst::clear_cache(max_age)
```

`clear_cache` takes the cache's write lock, so calling it in a loop while other
threads compile will starve them. Measured with four threads compiling and one
evicting as fast as it could, the compiles took 400 times longer. Call it
between batches of documents, not between documents.

### fonts are discovered once

Walking the system font directories takes around 95 ms, which for a small
document is far longer than the compile itself. It happens on the first compile
that needs it and the result is reused for the rest of the process, so a font
installed or removed later stays invisible until you say otherwise:

```ruby
Typst::clear_font_cache
```

Directories passed as `font_paths` are exempt: they are rescanned on every
compile, because `from_s` writes the fonts you hand it into a fresh temporary
directory each time. It only puts that directory on the font path when you
actually pass `fonts:`, so a `body:` or `zip:` compile without them is as fast
as any other.

## Contributors & Acknowledgements
typst-rb is based on [typst-py](https://github.com/messense/typst-py) by [messense](https://github.com/messense)\
clear_cache was contributed by [NRicciVestmark](https://github.com/NRicciVestmark)\
CI improvements were contributed by [am1006](https://github.com/am1006)\
Defect resolutions by [adam12](https://github.com/adam12) and [walterdavis](https://github.com/walterdavis)\
Design suggestions by [alec-c4](https://github.com/alec-c4)\
Compiler warnings were contributed by [TheSoloHacker47](https://github.com/TheSoloHacker47)

## License

This work is released under the Apache-2.0 license. A copy of the license is provided in the [LICENSE](./LICENSE) file.
