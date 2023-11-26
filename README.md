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
Typst::Html.new("readme.typ", "README").write("readme.html")

# Or return HTML content
markup = Typst::Html.new("readme.typ", "README").markup
# => "\n<!DOCTYPE html>\n<html>\n<head>\n<title>README</title>\n</head>\n<bo..."
```

## Contributors & Acknowledgements
typst-rb is based on [typst-py](https://github.com/messense/typst-py) by [messense](https://github.com/messense)

## License

This work is released under the Apache-2.0 license. A copy of the license is provided in the [LICENSE](./LICENSE) file.
