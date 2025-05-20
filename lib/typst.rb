def Typst(*options)
  Typst::Base.new(*options)
end

module Typst
  @@formats = {}

  def self.register_format(**format)
    @@formats.merge!(format)
  end
  
  def self.formats
    @@formats
  end
end


require "cgi"
require "pathname"
require "tmpdir"
require "zip/filesystem"

require_relative "typst/typst"
require_relative "base"
require_relative "query"
require_relative "document"
require_relative "formats/pdf"
require_relative "formats/svg"
require_relative "formats/png"
require_relative "formats/html"
require_relative "formats/html_experimental"
