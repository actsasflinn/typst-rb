module Typst
  class Html < Base
    def initialize(*options)
      super(*options)
      title = CGI::escapeHTML(@options[:title] || File.basename(@options[:file], ".*"))
      @compiled = HtmlDocument.new(Typst::_to_svg(*self.typst_args), title)
    end
  end

  class HtmlDocument < Document
    attr_accessor :title

    def initialize(bytes, title)
      super(bytes)
      self.title = title
    end

    def markup
      %{
  <!DOCTYPE html>
  <html>
  <head>
  <title>#{title}</title>
  </head>
  <body>
  #{pages.join("<br />")}
  </body>
  </html>
      }
    end
    alias_method :document, :markup
  end

  register_format(html: Html)
end
