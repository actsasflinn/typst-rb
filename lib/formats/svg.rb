module Typst
  class Svg < Base
    def initialize(*options)
      super(*options)
      @compiled = SvgDocument.new(Typst::_to_svg(*self.typst_args))
    end
  end
  class SvgDocument < Document; end

  register_format(svg: Svg)
end
