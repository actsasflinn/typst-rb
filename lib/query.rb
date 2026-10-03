module Typst
  class Query < Base
    attr_accessor :format

    def initialize(selector, input = nil, field: nil, one: false, format: "json", root: ".", font_paths: [], ignore_system_fonts: false, ignore_embedded_fonts: false, sys_inputs: {})
      self.format = format
      opts = { root: root, font_paths: font_paths, ignore_system_fonts: ignore_system_fonts, ignore_embedded_fonts: ignore_embedded_fonts, sys_inputs: sys_inputs }
      if input
        opts[:file] = input

        Typst::from(opts) do |options|
          from_options = options.slice(
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
            from_options[:system_fonts] = !options[:ignore_system_fonts] if from_options[:system_fonts].nil?
            from_options[:embedded_fonts] = !options[:ignore_embedded_fonts] if from_options[:embedded_fonts].nil?
            from_options[:fonts] = from_options[:fonts].values.collect{ |v| v.bytes }

            world = Typst::VirtualWorld.new(options[:body], **from_options)
            @result = world.query(selector, field, one, format)
        end
      end
    end

    def result(raw: false)
      case raw || format
        when "json" then JSON(@result)
        when "yaml" then YAML::safe_load(@result)
        else @result
      end
    end

    def to_s
      @result
    end
  end
end