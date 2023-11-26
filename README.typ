
#show link: underline
#show link: set text(blue)

= typst-rb

Ruby binding to #link("https://github.com/typst/typst")[typst], a new markup-based typesetting system that is powerful and easy to learn.

== Installation

```bash
gem install typst
```

== Usage

```ruby
require "typst"

# Compile `readme.typ` to PDF and save as `readme.pdf`
Typst::Pdf.new("readme.typ").write("readme.pdf")
# => #<Typst:0x0000000104bbe5e0 @font_paths=[], @input="hello.typ", @root="."> 

# Or return PDF content as an array of bytes
pdf_bytes = Typst::Pdf.new("readme.typ").bytes
# => [37, 80, 68, 70, 45, 49, 46, 55, 10, 37, 128 ...] 

# Or return PDF content as a string of bytes
document = Typst::Pdf.new("readme.typ").document
# => "%PDF-1.7\n%\x80\x80\x80\x80\n\n4 0 obj\n<<\n  /Type /Font\n  /Subtype ..." 

# Compile `readme.svg` to SVG and save as `readme.svg`
Typst::Svg.new("readme.typ").write("readme.svg")
# => #<Typst:0x0000000104bbe5e0 @font_paths=[], @input="hello.typ", @root="."> 

# Or return SVG content as an array of pages
pages = Typst::Svg.new("readme.typ").pages
# => ["<svg class=\"typst-doc\" viewBox=\"0 0 595.2764999999999 841.89105\" ..."
```

== Contributors & Acknowledgements
This is mostly a port of #link("https://github.com/messense/typst-py")[typst-py] by #link("https://github.com/messense")[messense]

== License

This work is released under the Apache-2.0 license. A copy of the license is provided in the #link("https://github.com/actsasflinn/typst-rb/blob/main/LICENSE")[LICENSE] file.