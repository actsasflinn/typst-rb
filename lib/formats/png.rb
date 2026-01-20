module Typst
  class Png < Base
    def initialize(*options)
      super(*options)
      @compiled = PngDocument.new(Typst::_to_png(*self.typst_png_args))
    end
  end
  class PngDocument < Document
    alias_method :write, :write_some
  end

  register_format(png: Png)
end
