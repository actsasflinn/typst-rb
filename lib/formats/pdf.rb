module Typst
  class Pdf < Base
    def initialize(*options)
      super(*options)
      #bytes, warnings = Typst::_to_pdf(*self.typst_pdf_args)
      @compiled = compile(:pdf)
    end
  end
  
  register_format(pdf: Pdf)
end
