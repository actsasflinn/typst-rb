module Typst
  class Svg < Base
    def initialize(*options)
      super(*options)
      bytes, warnings = Typst::_to_svg(*self.typst_pretty_args)
      @compiled = SvgDocument.new(bytes, warnings)
    end
  end
  class SvgDocument < Document
    alias_method :write, :write_some
  end

  register_format(svg: Svg)
end
