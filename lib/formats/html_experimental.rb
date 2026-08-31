module Typst
  class HtmlExperimental < Base
    def initialize(*options)
      super(*options)
      bytes, warnings = Typst::_to_html(*self.typst_pretty_args)
      @compiled = HtmlExperimentalDocument.new(bytes, warnings)
    end
  end
  class HtmlExperimentalDocument < Document
    alias_method :write, :write_one
  end

  register_format(html_experimental: HtmlExperimental)
end
