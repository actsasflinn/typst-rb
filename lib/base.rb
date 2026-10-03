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
      options[:ignore_system_fonts] ||= false
      options[:ignore_embedded_fonts] ||= false
      options[:pretty] ||= false
      options[:render_bleed] ||= false
    
      self.options = options
    end

    def typst_options
      [:file, :root, :font_paths, :ignore_system_fonts, :ignore_embedded_fonts, :render_bleed]
    end

    def typst_args(opts)
      options.values_at(*opts).append(options[:sys_inputs].map{ |k,v| [k.to_s,v.to_s] }.to_h)
    end

    def typst_pretty_args
      typst_args(typst_options.append(:pretty))
    end

    def typst_pdf_args
      options[:pdf_standards] ||= []
      opts = typst_options - [:render_bleed] + [:pretty]
      args = typst_args(opts)
      [*args, options[:pdf_standards]]
    end

    def typst_png_args
      [*typst_args(typst_options), options[:ppi]]
    end

    def self.from_s(main_source, **options)
      Typst::build_world_from_s(main_source, **options) do |opts|
        from_options = options.merge(opts)
        if from_options[:format]
          Typst::formats[from_options[:format]].new(**from_options)
          #Typst::VirtualWorld.new(main_source, **from_options)
        else
          new(**from_options)
          #Typst::VirtualWorld.new(main_source, **from_options)
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

    def pretty(pretty = true)
      self.options[:pretty] = pretty
      self
    end

    def ugly(pretty = false)
      self.pretty(pretty)
      self
    end

    def compile(format, **options)
      options = self.options.merge(options)

      if options.has_key?(:file)
        options[:body] = File.read(options[:file])
        fn = self.options.delete(:file)
        res = Typst::build_world_from_project(options[:file], **options) do |opts|
          opts.delete(:file)
          compile(format, **opts)
        end
        self.options[:file] = fn
        res
      elsif options.has_key?(:body)
        Typst::build_world_from_s(self.options[:body], **options) do |opts|
          from_options = options.merge(opts).slice(
            :fonts,
            :font_paths,
            :system_fonts,
            :embedded_fonts,
            :local_fonts,
            :package_path,
            :package_cache_path,
            :dependencies,
            :sys_inputs)

          from_options[:files] = from_options.delete(:dependencies)
          from_options[:system_fonts] = !opts[:ignore_system_fonts] if from_options[:system_fonts].nil?
          from_options[:embedded_fonts] = !opts[:ignore_embedded_fonts] if from_options[:embedded_fonts].nil?
          from_options[:fonts] = from_options[:fonts].values.collect{ |v| v.bytes }

          # puts from_options.inspect

          t = Typst::VirtualWorld.new(options[:body], **from_options)

          case format
            when :html,:html_experimental
              t.to_html(**options.merge(opts).slice(:pretty))
            when :pdf
              t.to_pdf(**options.merge(opts).slice(:pretty, :pdf_standards))
            when :png
              t.to_png(**options.merge(opts).slice(:render_bleed, :ppi))
            when :svg
              t.to_svg(**options.merge(opts).slice(:pretty, :render_bleed))
            else
              raise "Invalid format"
          end
        end
      elsif options.has_key?(:zip)
        main_file = options[:main_file]
        fn = self.options.delete(:file)
        res = Typst::build_world_from_zip(options[:zip], main_file, **options) do |opts|
          opts.delete(:zip)  
          compile(format, **opts)
        end
        self.options[:file] = fn
        res
      else
        raise "No input given"
      end
    end

    def query(selector, field: nil, one: false, format: "json")
      query_options = { field: field, one: one, format: format }

      if self.options.has_key?(:file)
        Typst::Query.new(selector, self.options[:file], **query_options.merge(self.options.slice(:root, :font_paths, :ignore_system_fonts, :ignore_embedded_fonts, :sys_inputs)))
      elsif self.options.has_key?(:body)
        Typst::build_world_from_s(self.options[:body], **self.options) do |opts|
          Typst::Query.new(selector)
        end
      elsif self.options.has_key?(:zip)
        options.delete(:file)
        self.options.delete(:file)

        Typst::build_world_from_zip(self.options[:zip], **self.options) do |opts|
          Typst::Query.new(selector)
        end
      else
        raise "No input given"
      end
    end
  end
end
