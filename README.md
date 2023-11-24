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

# Compile `hello.typ` to PDF and save as `hello.pdf`
Typst::compile("hello.typ", "hello.pdf", ".", [])

# Or return PDF content as bytes
pdf_bytes = Typst.compile("hello.typ", nil, ".", [])
```

## License

This work is released under the Apache-2.0 license. A copy of the license is provided in the [LICENSE](./LICENSE) file.
