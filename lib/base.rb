module Typst
  class Base
    attr_accessor :options
    attr_accessor :compiled

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
      elsif options.has_key?(:body)
        raise "Empty body" if options[:body].to_s.empty?
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

      options[:dependencies] ||= {}
      options[:fonts] ||= {}
      options[:sys_inputs] ||= {}
    
      self.options = options
    end

    def typst_args
      [options[:file], options[:root], options[:font_paths], File.dirname(__FILE__), false, options[:sys_inputs].map{ |k,v| [k.to_s,v.to_s] }.to_h]
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

        if options[:format]
          Typst::formats[options[:format]].new(**options)
        else
          new(**options)
        end
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

    def with_dependencies(dependencies)
      self.options[:dependencies] = self.options[:dependencies].merge(dependencies)
      self
    end

    def with_fonts(fonts)
      self.options[:fonts] = self.options[:fonts].merge(fonts)
      self
    end

    def with_inputs(inputs)
      self.options[:sys_inputs] = self.options[:sys_inputs].merge(inputs)
      self
    end

    def with_font_paths(font_paths)
      self.options[:font_paths] = self.options[:font_paths] + font_paths
      self
    end

    def with_root(root)
      self.options[:root] = root
      self
    end

    def compile(format, **options)
      raise "Invalid format" if Typst::formats[format].nil?

      options = self.options.merge(options)

      if options.has_key?(:file)
        Typst::formats[format].new(**options).compiled
      elsif options.has_key?(:body)
        Typst::formats[format].from_s(options[:body], **options).compiled
      elsif options.has_key?(:zip)
        Typst::formats[format].from_zip(options[:zip], options[:main_file], **options).compiled
      else
        raise "No input given"
      end
    end

    def write(output)
      STDERR.puts "DEPRECATION WARNING: this method will go away in a future version"
      compiled.write(output)
    end

    def document
      STDERR.puts "DEPRECATION WARNING: this method will go away in a future version"
      compiled.document
    end

    def bytes
      STDERR.puts "DEPRECATION WARNING: this method will go away in a future version"
      compiled.bytes
    end

    def pages
      STDERR.puts "DEPRECATION WARNING: this method will go away in a future version"
      compiled.pages
    end
  end
end
