require_relative "typst/typst"
require "cgi"
module Typst
  class Pdf
    attr_accessor :input
    attr_accessor :root
    attr_accessor :font_paths
    attr_accessor :bytes
    
    def initialize(input, root: ".", font_paths: [])
      self.input = input
      self.root = root
      self.font_paths = font_paths

      document
    end

    def to_pdf
      Typst::_to_pdf(input, root, font_paths, File.dirname(__FILE__))
    end

    def update
      @bytes = to_pdf
    end

    def bytes
      @bytes ||= to_pdf
    end

    def document
      bytes.pack("C*").to_s
    end

    def write(output)
      File.open(output, "w"){ |f| f.write(document) }
    end
  end

  class Svg
    attr_accessor :input
    attr_accessor :root
    attr_accessor :font_paths
    attr_accessor :pages
    
    def initialize(input, root: ".", font_paths: [])
      self.input = input
      self.root = root
      self.font_paths = font_paths
    
      pages
    end

    def to_svg
      Typst::_to_svg(input, root, font_paths, File.dirname(__FILE__))
    end

    def update
      @pages = to_svg
    end

    def pages
      @pages ||= to_svg
    end

    def write(output)
      if pages.size > 1
        pages.each_with_index do |page, i|
          if output.include?("{{n}}")
            file_name = output.gsub("{{n}}", (i+1).to_s)
          else
            file_name = File.basename(output, File.extname(output)) + "_" + i.to_s
            file_name = file_name + File.extname(output)
          end
          File.open(file_name, "w"){ |f| f.write(page) }
        end
      elsif pages.size == 1
        File.open(output, "w"){ |f| f.write(pages[0]) }
      else
      end
    end
  end

  class Html
    attr_accessor :title
    attr_accessor :svg
    attr_accessor :html

    def initialize(input, title = nil, root: ".", font_paths: [])
      title = title || File.basename(input, File.extname(input))
      @title = CGI::escapeHTML(title)
      @svg = Svg.new(input, root: root, font_paths: font_paths)
    end

    def markup
      %{
<!DOCTYPE html>
<html>
<head>
<title>#{title}</title>
</head>
<body>
#{svg.pages.join("<br />")}
</body>
</html>
      }
    end

    def write(output)
      File.open(output, "w"){ |f| f.write(markup) }
    end
  end
end