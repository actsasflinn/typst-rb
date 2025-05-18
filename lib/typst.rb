require_relative "typst/typst"
require "cgi"
require "pathname"
require "tmpdir"
require "zip/filesystem"

def Typst(*options)
  Typst::Base.new(*options)
end

module Typst
  class Base
    attr_accessor :options
    #attr_accessor :typst_args

    def initialize(*options)
      if options.size.zero?
        raise "No options given"
      elsif options.first.is_a?(String)
        file, options = options
        options ||= {}
        options[:file] = file
      elsif options.first.is_a?(Hash)
        options = options.first
      end

      if options.has_key?(:file)
        raise "Can't find file" unless File.exist?(options[:file])
      elsif options.has_key?(:template)
        raise "Empty template" if options[:template].to_s.empty?
      elsif options.has_key?(:zip)
        raise "Can't find zip" unless File.exist?(options[:zip])
      else
        raise "No input given"
      end

      root = Pathname.new(options[:root] || ".").expand_path
      raise "Invalid path for root" unless root.exist?
      options[:root] = root.to_s

      font_paths = (options[:font_paths] || []).collect{ |fp| Pathname.new(fp).expand_path }
      options[:font_paths] = font_paths.collect(&:to_s)

      options[:sys_inputs] ||= {}
    
      self.options = options
    end

    def typst_args
      typst_args = [options[:file], options[:root], options[:font_paths], File.dirname(__FILE__), false, options[:sys_inputs].map{ |k,v| [k.to_s,v.to_s] }.to_h]
    end

    def write(output, segments = [document])
      if segments.nil? || !segments.is_a?(Array) || segments.size.zero?
        raise "Nothing to write"
      elsif segments.size == 1
        File.write(output, segments.first, mode: "wb")
      elsif segments.size > 1
        segments.each_with_index do |data, i|
          if output.include?("{{n}}")
            fn = output.gsub("{{n}}", (i+1).to_s)
          else
            fn = File.basename(output, File.extname(output)) + "_" + i.to_s
            fn = fn + File.extname(output)
          end
          write(fn, [data])
        end
      end
    end

    def self.from_s(main_source, **options)
      dependencies = options[:dependencies] ||= {}
      fonts = options[:fonts] ||= {}

      Dir.mktmpdir do |tmp_dir|
        tmp_main_file = Pathname.new(tmp_dir).join("main.typ")
        File.write(tmp_main_file, main_source)

        dependencies.each do |dep_name, dep_source|
          tmp_dep_file = Pathname.new(tmp_dir).join(dep_name)
          File.write(tmp_dep_file, dep_source)
        end

        relative_font_path = Pathname.new(tmp_dir).join("fonts")
        fonts.each do |font_name, font_bytes|
          Pathname.new(relative_font_path).mkpath
          tmp_font_file = relative_font_path.join(font_name)
          File.write(tmp_font_file, font_bytes)
        end

        options[:file] = tmp_main_file
        options[:root] = tmp_dir
        options[:font_paths] = [relative_font_path]

        new(**options)
      end
    end

    def self.from_zip(zip_file_path, main_file = nil, **options)
      options[:dependencies] ||= {}
      options[:fonts] ||= {}

      Zip::File.open(zip_file_path) do |zipfile|
        file_names = zipfile.dir.glob("*").collect{ |f| f.name }
        case
          when file_names.include?(main_file) then tmp_main_file = main_file
          when file_names.include?("main.typ") then tmp_main_file = "main.typ"
          when file_names.size == 1 then tmp_main_file = file_names.first
          else raise "no main file found"
        end
        main_source = zipfile.file.read(tmp_main_file)
        file_names.delete(tmp_main_file)
        file_names.delete("fonts/")

        file_names.each do |dep_name|
          options[:dependencies][dep_name] = zipfile.file.read(dep_name)
        end

        font_file_names = zipfile.dir.glob("fonts/*").collect{ |f| f.name }
        font_file_names.each do |font_name|
          options[:fonts][Pathname.new(font_name).basename.to_s] = zipfile.file.read(font_name)
        end

        options[:main_file] = tmp_main_file

        from_s(main_source, **options)
      end
    end

    def with_inputs(inputs)
      self.options[:sys_inputs] = self.options[:sys_inputs].merge(inputs)
      self
    end

    def to(format, **options)
      formats = { pdf: Pdf, svg: Svg, png: Png, html: Html, html_experimental: HtmlExperimental }
      raise "Invalid format" if formats[format].nil?

      options = self.options.merge(options)

      if options.has_key?(:file)
        formats[format].new(**options)
      elsif options.has_key?(:template)
        formats[format].from_s(options[:template], **options)
      elsif options.has_key?(:zip)
        formats[format].from_zip(options[:zip], **options)
      else
        raise "No input given"
      end
    end
  end

  class Pdf < Base
    attr_accessor :bytes
    
    def initialize(*options)
      super(*options)
      @bytes = Typst::_to_pdf(*self.typst_args)[0]
    end

    def document
      bytes.pack("C*").to_s
    end
  end

  class Svg < Base
    attr_accessor :pages
    
    def initialize(*options)
      super(*options)
      @pages = Typst::_to_svg(*self.typst_args).collect{ |page| page.pack("C*").to_s }
    end

    def write(output, segments = self.pages)
      super(output, segments)
    end
  end

  class Png < Base
    attr_accessor :pages

    def initialize(*options)
      super(*options)
      @pages = Typst::_to_png(*self.typst_args).collect{ |page| page.pack("C*").to_s }
    end

    def write(output, segments = self.pages)
      super(output, segments)
    end
  end

  class Html < Svg
    attr_accessor :title

    def initialize(*options)
      super(*options)

      title = self.options[:title] || File.basename(self.options[:file], File.extname(self.options[:file]))
      self.title = CGI::escapeHTML(title)
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

  class HtmlExperimental < Base
    attr_accessor :bytes

    def initialize(*options)
      super(*options)
      options = self.options

      @bytes = Typst::_to_html(*self.typst_args)[0]
    end

    def document
      bytes.pack("C*").to_s
    end
    alias_method :markup, :document
  end

  class Query < Base
    attr_accessor :format
    attr_accessor :result

    def initialize(selector, input, field: nil, one: false, format: "json", root: ".", font_paths: [], sys_inputs: {})
      super(input, root: root, font_paths: font_paths, sys_inputs: sys_inputs)
      self.format = format
      self.result = Typst::_query(selector, field, one, format, self.input, self.root, self.font_paths, File.dirname(__FILE__), false, sys_inputs)
    end

    def result(raw: false)
      case raw || format
        when "json" then JSON(@result)
        when "yaml" then YAML::safe_load(@result)
        else @result
      end
    end
  end
end
