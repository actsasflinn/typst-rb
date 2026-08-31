module Typst
  class Pdf < Base
    def initialize(*options)
      super(*options)
      bytes, warnings = Typst::_to_pdf(*self.typst_pdf_args)
      @compiled = PdfDocument.new(bytes, warnings)
    end
  end
  class PdfDocument < Document
    alias_method :write, :write_one
  end

  register_format(pdf: Pdf)
end
