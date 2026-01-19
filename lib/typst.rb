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

  def self.build_world_from_s(main_source, **options, &blk)
    dependencies = options[:dependencies] ||= {}
    fonts = options[:fonts] ||= {}

    Dir.mktmpdir do |tmp_dir|
      tmp_main_file = Pathname.new(tmp_dir).join("main.typ")
      File.write(tmp_main_file, main_source)

      dependencies.each do |dep_name, dep_source|
        tmp_dep_file = Pathname.new(tmp_dir).join(dep_name)
        File.write(tmp_dep_file, dep_source)
      end

      relative_font_path = Pathname.new(tmp_dir).join("fonts")
      relative_font_path.mkpath
      fonts.each do |font_name, font_bytes|
        tmp_font_file = relative_font_path.join(font_name)
        File.write(tmp_font_file, font_bytes)
      end

      options[:file] = tmp_main_file
      options[:root] = tmp_dir
      options[:font_paths] = [relative_font_path]

      blk.call(options)
    end
  end

  def self.build_world_from_zip(zip_file_path, main_file = "main.typ", **options, &blk)
    options[:dependencies] ||= {}
    options[:fonts] ||= {}

    Zip::File.open(zip_file_path) do |zipfile|
      file_names = zipfile.dir.glob("*").collect{ |f| f.name }
      case
        when file_names.include?(main_file) then tmp_main_file = main_file
        when file_names.include?("main.typ") then tmp_main_file = "main.typ"
        when file_names.size == 1 then tmp_main_file = file_names.first
        else raise "no main file found"
      end
      main_source = zipfile.file.read(tmp_main_file)
      file_names.delete(tmp_main_file)
      file_names.delete("fonts/")

      file_names.each do |dep_name|
        options[:dependencies][dep_name] = zipfile.file.read(dep_name)
      end

      font_file_names = zipfile.dir.glob("fonts/*").collect{ |f| f.name }
      font_file_names.each do |font_name|
        options[:fonts][Pathname.new(font_name).basename.to_s] = zipfile.file.read(font_name)
      end

      options[:main_file] = tmp_main_file

      build_world_from_s(main_source, **options, &blk)
    end
  end
end

require "cgi"
require "pathname"
require "tmpdir"
require "zip/filesystem"
require "json"
require "yaml"

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
