module Typst
  class Document
    # Diagnostics the compiler emitted while *succeeding*. An unknown font
    # family is the common one: Typst substitutes another face, the document
    # generates, and nothing tells you unless you look here.
    attr_accessor :warnings

    # One binary String per page, as the extension returns them. An Array of
    # Integers per page, what it returned before, is still accepted.
    def initialize(pages, warnings = [])
      @pages = binary_pages(pages)
      @warnings = warnings
    end

    def warnings?
      !warnings.empty?
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

    # One binary (ASCII-8BIT) String per page; PDF and HTML output is a
    # single page. Each call returns new Strings, as it did when it packed
    # them every time. They share their bytes with the document until
    # changed, so changing one (force_encoding, say) leaves the document as
    # it was compiled.
    def pages
      @pages.collect(&:dup)
    end

    # The pages as arrays of Integers. Prefer #pages: this makes a Ruby
    # Integer of every byte.
    def bytes
      @pages.collect(&:bytes)
    end

    # Replaces the pages, given as #bytes returns them or as Strings.
    def bytes=(pages)
      @pages = binary_pages(pages)
    end

    def document
      pages.size == 1 ? pages.first : pages
    end

    private

    def binary_pages(pages)
      pages.collect { |page| page.is_a?(Array) ? page.pack("C*") : page }
    end
  end
end