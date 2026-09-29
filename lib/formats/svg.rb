module Typst
  class Svg < Base
    def initialize(*options)
      super(*options)
      #bytes, warnings = Typst::_to_svg(*self.typst_pretty_args)

      # world_options = options.slice(
      #   :fonts,
      #   :font_paths,
      #   :system_fonts,
      #   :embedded_fonts,
      #   :local_fonts,
      #   :package_path,
      #   :package_cache_path,
      #   :files,
      #   :sys_inputs)
      
      # world_options[:system_fonts] = !options[:ignore_system_fonts] if options[:system_fonts].nil?
      # world_options[:embedded_fonts] = !options[:ignore_embedded_fonts] if options[:embedded_fonts].nil?
      # world_options[:fonts] = world_options[:fonts].values

      # Typst::VirtualWorld.new(options)

      @compiled = compile(:svg)
    end
  end

  register_format(svg: Svg)
end
