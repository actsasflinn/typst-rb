module Typst
  class Document
    attr_accessor :bytes

    def initialize(bytes)
      @bytes = bytes
    end

    def write_some(filename)
      if pages.size == 1
        write_one(filename)
      else
        write_paged(filename)
      end
    end

    def write_one(filename)
      File.write(filename, pages.first, mode: "wb")
    end

    def write_paged(base_filename)
      pages.each_with_index do |page, i|
        paged_filename = File.basename(base_filename, ".*") + "_{{n}}" + File.extname(base_filename) unless base_filename.include?("{{n}}")
        paged_filename = paged_filename.gsub("{{n}}", (i+1).to_s)
        File.write(paged_filename, page, mode: "wb")
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