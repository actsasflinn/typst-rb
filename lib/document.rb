module Typst
  class Document
    attr_accessor :bytes

    def initialize(bytes)
      @bytes = bytes
    end

    def write(out)
      if pages.size == 1
        File.write(out, pages.first, mode: "wb")
      else
        pages.each_with_index do |page, i|
          fn = File.basename(out, ".*") + "_{{n}}" + File.extname(out) unless out.include?("{{n}}")
          fn = fn.gsub("{{n}}", (i+1).to_s)
          File.write(fn, page, mode: "wb")
        end
      end
    end

    def pages
      bytes.collect{ |page| page.pack("C*").to_s }
    end

    def document
      pages.size == 1 ? pages.first : pages
    end
  end
end