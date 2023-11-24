= typst-rb

Ruby binding to
#link("https://github.com/typst/typst")[
  typst
], a new markup-based typesetting system that is powerful and easy to learn.

== Installation

```bash
gem install typst
```

== Usage

```ruby
require "typst"

# Compile `hello.typ` to PDF and save as `hello.pdf`
Typst::compile("readme.typ", "readme.pdf", ".", [])

# Or return PDF content as bytes
pdf_bytes = Typst.compile("readme.typ", nil, ".", [])
```

== License

This work is released under the Apache-2.0 license. A copy of the license is provided in the
#link("LICENSE")[
  LICENSE
] file.