require_relative "typst/typst"
require "cgi"
require "pathname"
require "tmpdir"

module Typst
  class Base
    attr_accessor :input
    attr_accessor :root
    attr_accessor :font_paths

    def initialize(input, root: ".", font_paths: [])
      self.input = input
      self.root = Pathname.new(root).expand_path.to_s
      self.font_paths = font_paths.collect{ |fp| Pathname.new(fp).expand_path.to_s }
    end

    def write(output)
      File.open(output, "w"){ |f| f.write(document) }
    end

    def self.from_s(main_source, dependencies: {}, fonts: {})
    dependencies = {} if dependencies.nil?
    fonts = {} if fonts.nil?
    Dir.mktmpdir do |tmp_dir|
        tmp_main_file = Pathname.new(tmp_dir).join("main.typ")
        File.write(tmp_main_file, main_source)

        dependencies.each do |dep_name, dep_source|
          tmp_dep_file = Pathname.new(tmp_dir).join(dep_name)
          File.write(tmp_dep_file, dep_source)
        end

        relative_font_path = Pathname.new(tmp_dir).join("fonts")
        puts fonts
        fonts.each do |font_name, font_bytes|
          Pathname.new(relative_font_path).mkpath
          tmp_font_file = relative_font_path.join(font_name)
          File.write(tmp_font_file, font_bytes)
        end

        new(tmp_main_file, root: tmp_dir, font_paths: [relative_font_path])
      end
    end
  end

  class Pdf < Base
    attr_accessor :bytes
    
    def initialize(input, root: ".", font_paths: [])
      super(input, root: root, font_paths: font_paths)
      @bytes = Typst::_to_pdf(self.input, self.root, self.font_paths, File.dirname(__FILE__))
    end

    def document
      bytes.pack("C*").to_s
    end
  end

  class Svg < Base
    attr_accessor :pages
    
    def initialize(input, root: ".", font_paths: [])
      super(input, root: root, font_paths: font_paths)
      @pages = Typst::_to_svg(self.input, self.root, self.font_paths, File.dirname(__FILE__))
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

  class Html < Base
    attr_accessor :title
    attr_accessor :svg
    attr_accessor :html

    def initialize(input, title: nil, root: ".", font_paths: [])
      super(input, root: root, font_paths: font_paths)
      title = title || File.basename(input, File.extname(input))
      self.title = CGI::escapeHTML(title)
      self.svg = Svg.new(self.input, root: self.root, font_paths: self.font_paths)
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
    alias_method :document, :markup
  end
end