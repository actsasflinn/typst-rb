module Typst
  class Png < Base
    def initialize(*options)
      super(*options)
      @compiled = PngDocument.new(Typst::_to_png(*self.typst_args))
    end
  end
  class PngDocument < Document; end

  register_format(png: Png)
end
