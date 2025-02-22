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

# Compile `readme.typ` to PDF and save as `readme.pdf`
Typst::Pdf.new("readme.typ").write("readme.pdf")

# Or return PDF content as an array of bytes
pdf_bytes = Typst::Pdf.new("readme.typ").bytes
# => [37, 80, 68, 70, 45, 49, 46, 55, 10, 37, 128 ...] 

# Or return PDF content as a string of bytes
document = Typst::Pdf.new("readme.typ").document
# => "%PDF-1.7\n%\x80\x80\x80\x80\n\n4 0 obj\n<<\n  /Type /Font\n  /Subtype ..." 

# Compile `readme.typ` to SVG and save as `readme.svg`
Typst::Svg.new("readme.typ").write("readme.svg")

# Or return SVG content as an array of pages
pages = Typst::Svg.new("readme.typ").pages
# => ["<svg class=\"typst-doc\" viewBox=\"0 0 595.2764999999999 841.89105\" ..."

# Compile `readme.typ` to SVG and save as `readme.html`
Typst::Html.new("readme.typ", title: "README").write("readme.html")

# Or return HTML content
markup = Typst::Html.new("readme.typ", title: "README").markup
# => "\n<!DOCTYPE html>\n<html>\n<head>\n<title>README</title>\n</head>\n<bo..."

# Compile from a string to PDF
t = Typst::Pdf.from_s(%{hello world})

# Compile from a string to SVG
t = Typst::Svg.from_s(%{hello world})

# Compile from a string to PDF
t = Typst::Html.from_s(%{hello world})

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

t = Typst::Pdf.from_s(main, dependencies: { "template.typ" => template, "icon.svg" => icon }, fonts: { "Example.ttf" => font_bytes })

# From a zip file that includes a main.typ
# zip file include flat dependencies included and a fonts directory
Typst::Pdf::from_zip("working_directory.zip")

# From a zip with a named main typst file
Typst::Pdf::from_zip("working_directory.zip", "hello.typ")
```

## Contributors & Acknowledgements
typst-rb is based on [typst-py](https://github.com/messense/typst-py) by [messense](https://github.com/messense)

## License

This work is released under the Apache-2.0 license. A copy of the license is provided in the [LICENSE](./LICENSE) file.
