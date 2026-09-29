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

  def self.build_world_from_project(main_filename, **options, &blk)
    files = Dir.glob(File.join(options[:root], "*"))
    exts = [".typ", ".svg", ".png", ".json"]
    files.filter{ |fn| exts.any?{ |ext| File.extname(fn) == ext } }.each do |fil|
      options[:dependencies][File.basename(fil)] = File.read(fil) if File.file?(fil)
    end

    options[:fonts] ||= {}
    fonts = Dir.glob(File.join(options[:root], "fonts", "*"))
    fonts.each do |font|
      options[:fonts][File.basename(font)] = File.read(font) if File.file?(font)
    end

    self.build_world_from_s(File.read(main_filename), **options, &blk)
  end

  def self.build_world_from_s(main_source, **options, &blk)
    options[:body] = main_source
    dependencies = options[:dependencies] || {}
    options[:dependencies] = dependencies.collect{ |k,v| [k,v.is_a?(String) ? v.bytes : v] }.to_h

    fonts = options[:fonts] ||= {}
    fonts.each do |font|
      options[:fonts][File.basename(font)] = File.read(font)
    end

    blk.call(options)
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

      #options[:main_file] = tmp_main_file

      build_world_from_s(main_source, **options, &blk)
    end
  end
end

require "cgi/escape"
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
# require_relative "query"
# require_relative "document"
require_relative "formats/pdf"
require_relative "formats/svg"
require_relative "formats/png"
require_relative "formats/html_experimental"
