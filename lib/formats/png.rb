module Typst
  class Png < Base
    def initialize(*options)
      super(*options)
      #bytes, warnings = Typst::_to_png(*self.typst_png_args)
      @compiled = compile(:png)
    end
  end

  register_format(png: Png)
end
