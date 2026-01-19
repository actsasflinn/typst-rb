module Typst
  class Query < Base
    attr_accessor :format

    def initialize(selector, input, field: nil, one: false, format: "json", root: ".", font_paths: [], resource_path: ".", ignore_system_fonts: false, sys_inputs: {})
      self.format = format
      @result = Typst::_query(selector, field, one, format, input, root, font_paths, resource_path, ignore_system_fonts, sys_inputs)
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