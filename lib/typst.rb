require "cgi"
require "pathname"
require "tmpdir"
require "zip/filesystem"

require_relative "typst/typst"
require_relative "base"
require_relative "query"
require_relative "document"
require_relative "pdf"
require_relative "svg"
require_relative "png"
require_relative "html"
require_relative "html_experimental"

def Typst(*options)
  Typst::Base.new(*options)
end
