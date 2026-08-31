module Typst
  class Png < Base
    def initialize(*options)
      super(*options)
      bytes, warnings = Typst::_to_png(*self.typst_png_args)
      @compiled = PngDocument.new(bytes, warnings)
    end
  end
  class PngDocument < Document
    alias_method :write, :write_some
  end

  register_format(png: Png)
end
