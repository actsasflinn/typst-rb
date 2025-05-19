module Typst
  class Query < Base
    attr_accessor :format

    def initialize(selector, input, field: nil, one: false, format: "json", root: ".", font_paths: [], sys_inputs: {})
      super(input, root: root, font_paths: font_paths, sys_inputs: sys_inputs)
      self.format = format
      @result = Typst::_query(selector, field, one, format, input, root, font_paths, File.dirname(__FILE__), false, sys_inputs)
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