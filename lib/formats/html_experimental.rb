module Typst
  class HtmlExperimental < Base
    def initialize(*options)
      super(*options)
      @compiled = HtmlExperimentalDocument.new(Typst::_to_html(*self.typst_args))
    end
  end
  class HtmlExperimentalDocument < Document; end

  register_format(html_experimental: HtmlExperimental)
end
