module Typst
  class Pdf < Base
    def initialize(*options)
      super(*options)
      @compiled = PdfDocument.new(Typst::_to_pdf(*self.typst_args))
    end
  end
  class PdfDocument < Document; end

  register_format(pdf: Pdf)
end
