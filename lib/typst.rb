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

  def self.clear_cache(max_age = 0)
    Typst::_clear_cache(max_age)
  end
end

require "cgi"
require "pathname"
require "tmpdir"
require "zip/filesystem"

begin
  # native precompiled gems package shared libraries in <gem_dir>/lib/typst/<ruby_version>
  RUBY_VERSION =~ /(\d+\.\d+)/
  require_relative "typst/#{Regexp.last_match(1)}/typst"
rescue LoadError
  require_relative "typst/typst"
end

require_relative "base"
require_relative "query"
require_relative "document"
require_relative "formats/pdf"
require_relative "formats/svg"
require_relative "formats/png"
require_relative "formats/html_experimental"
