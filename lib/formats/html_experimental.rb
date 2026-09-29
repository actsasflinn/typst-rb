module Typst
  class HtmlExperimental < Base
    def initialize(*options)
      super(*options)
      #bytes, warnings = Typst::_to_html(*self.typst_pretty_args)
      @compiled = compile(:html_experimental)
    end
  end

  register_format(html_experimental: HtmlExperimental)
end
