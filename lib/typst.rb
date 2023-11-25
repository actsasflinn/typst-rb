require_relative "typst/typst"

class Typst
  attr_accessor :input
  attr_accessor :root
  attr_accessor :font_paths
  attr_accessor :bytes
  
  def initialize(input, root: ".", font_paths: [])
    self.input = input
    self.root = root
    self.font_paths = font_paths
  
    bytes()
  end

  def compile
    Typst::_compile(input, root, font_paths)
  end

  def update
    @bytes = compile
  end

  def bytes
    @bytes ||= compile
  end

  def document
    bytes.pack("C*").to_s
  end

  def write(output)
    File.open(output, "w"){ |f| f.write(document) }
  end
end
