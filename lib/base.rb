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
      options[:resource_path] ||= File.dirname(__FILE__)
      options[:ignore_system_fonts] ||= false
    
      self.options = options
    end

    def typst_args
      [options[:file], options[:root], options[:font_paths], options[:resource_path], options[:ignore_system_fonts], options[:sys_inputs].map{ |k,v| [k.to_s,v.to_s] }.to_h]
    end

    def typst_png_args
      [*typst_args, options[:ppi]]
    end

    def self.from_s(main_source, **options)
      Typst::build_world_from_s(main_source, **options) do |opts|
        from_options = options.merge(opts)
        if from_options[:format]
          Typst::formats[from_options[:format]].new(**from_options)
        else
          new(**from_options)
        end
      end
    end

    def self.from_zip(zip_file_path, main_file = "main.typ", **options)
      Typst::build_world_from_zip(zip_file_path, main_file, **options) do |opts|
        from_options = options.merge(opts)
        if from_options[:format]
          Typst::formats[from_options[:format]].new(**from_options)
        else
          new(**from_options)
        end
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
        Typst::build_world_from_s(self.options[:body], **options) do |opts|
          Typst::formats[format].new(**options.merge(opts)).compiled
        end
      elsif options.has_key?(:zip)
        main_file = options[:main_file]
        Typst::build_world_from_zip(options[:zip], main_file, **options) do |opts|
          Typst::formats[format].new(**options.merge(opts)).compiled
        end
      else
        raise "No input given"
      end
    end

    def query(selector, field: nil, one: false, format: "json")
      query_options = { field: field, one: one, format: format }

      if self.options.has_key?(:file)
        Typst::Query.new(selector, self.options[:file], **query_options.merge(self.options.slice(:root, :font_paths, :resource_path, :ignore_system_fonts, :sys_inputs)))
      elsif self.options.has_key?(:body)
        Typst::build_world_from_s(self.options[:body], **self.options) do |opts|
          Typst::Query.new(selector, opts[:file], **query_options.merge(opts.slice(:root, :font_paths, :resource_path, :ignore_system_fonts, :sys_inputs)))
        end
      elsif self.options.has_key?(:zip)
        Typst::build_world_from_zip(self.options[:zip], **self.options) do |opts|
          Typst::Query.new(selector, opts[:file], **query_options.merge(opts.slice(:root, :font_paths, :resource_path, :ignore_system_fonts, :sys_inputs)))
        end
      else
        raise "No input given"
      end
    end
  end
end
