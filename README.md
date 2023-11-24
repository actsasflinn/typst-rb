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
Typst.new("readme.typ").write("readme.pdf")
# => #<Typst:0x0000000104bbe5e0 @font_paths=[], @input="hello.typ", @root="."> 

# Or return PDF content as an array of bytes
pdf_bytes = Typst.new("readme.typ").bytes
# => [37, 80, 68, 70, 45, 49, 46, 55, 10, 37, 128 ...] 

# Or return PDF content as a string of bytes
document = Typst.new("readme.typ").document
# => "%PDF-1.7\n%\x80\x80\x80\x80\n\n4 0 obj\n<<\n  /Type /Font\n  /Subtype ..." 
```

## Contributors & Acknowledgements
This is mostly a port of [typst-py](https://github.com/messense/typst-py) by [messense](https://github.com/messense)

## License

This work is released under the Apache-2.0 license. A copy of the license is provided in the [LICENSE](./LICENSE) file.
