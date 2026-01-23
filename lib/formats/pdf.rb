module Typst
  class Pdf < Base
    def initialize(*options)
      super(*options)
      @compiled = PdfDocument.new(Typst::_to_pdf(*self.typst_pdf_args))
    end
  end
  class PdfDocument < Document
    alias_method :write, :write_one
  end

  register_format(pdf: Pdf)
end
