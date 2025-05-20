require "test/unit"
require_relative "../lib/typst"

Dir.chdir(Pathname.new(__FILE__).dirname.to_s)

class TypstTest < Test::Unit::TestCase

  def test_pdf
    assert {
      Typst::Pdf.new("test.typ")
      Typst("test.typ").compile(:pdf)
    }
  end

  def test_pdf_assets
    assert {
      Typst::Pdf.new("template_with_font_and_icon/main.typ", root: "#{File.dirname(__FILE__)}/template_with_font_and_icon")
      Typst("template_with_font_and_icon/main.typ", root: "#{File.dirname(__FILE__)}/template_with_font_and_icon").compile(:pdf)
      Typst("template_with_font_and_icon/main.typ").with_root("#{File.dirname(__FILE__)}/template_with_font_and_icon").compile(:pdf)
    }
  end

  def test_pdf_without_font
    assert {
      !Typst::Pdf.new("template_with_font_and_icon/main.typ", root: "#{File.dirname(__FILE__)}/template_with_font_and_icon").document.include?("Fasthand")
      !Typst("template_with_font_and_icon/main.typ", root: "#{File.dirname(__FILE__)}/template_with_font_and_icon").compile(:pdf).document.include?("Fasthand")
      !Typst("template_with_font_and_icon/main.typ").with_root("#{File.dirname(__FILE__)}/template_with_font_and_icon").compile(:pdf).document.include?("Fasthand")
    }
  end

  def test_pdf_font
    assert {
      Typst::Pdf.new("template_with_font_and_icon/main.typ", root: "#{File.dirname(__FILE__)}/template_with_font_and_icon", font_paths: ["fonts/Fasthand/Release/ttf"]).document.include?("Fasthand")
      Typst("template_with_font_and_icon/main.typ", root: "#{File.dirname(__FILE__)}/template_with_font_and_icon", font_paths: ["fonts/Fasthand/Release/ttf"]).compile(:pdf).document.include?("Fasthand")
      Typst("template_with_font_and_icon/main.typ").with_root("#{File.dirname(__FILE__)}/template_with_font_and_icon").with_font_paths(["fonts/Fasthand/Release/ttf"]).compile(:pdf).document.include?("Fasthand")
    }
  end

  def test_svg
    assert {
      Typst::Svg.new("test.typ")
      Typst("test.typ").compile(:svg)
    }
  end

  def test_html
    assert {
      Typst::Html.new("test.typ")
      Typst("test.typ").compile(:html)
    }
  end

  def test_from_s
    assert {
      Typst::Pdf.from_s(%{hello world}, dependencies: nil, fonts: nil)
      Typst::Base.from_s(%{hello world}, format: :pdf, dependencies: nil, fonts: nil)
      Typst::Base.from_s(%{hello world}, format: :pdf)
      Typst(body: %{hello world}, dependencies: nil, fonts: nil).compile(:pdf)
      Typst(body: %{hello world}).compile(:pdf)
    }
  end

  def test_from_s_complex
    assert {
      main = %{
#import "template.typ": *

#show: template.with()

#lorem(50)

#image("icon.svg")
    }

      template = %{
#let template(body) = {
  set text(12pt, font: "Example")
  body
}
    }

      icon = File.read("#{File.dirname(__FILE__)}/template_with_font_and_icon/monkey.svg")
      font_bytes = File.read("#{File.dirname(__FILE__)}//fonts/Fasthand/Release/ttf/Fasthand-Regular.ttf")

      Typst::Pdf.from_s(main, dependencies: { "template.typ" => template, "icon.svg" => icon }, fonts: { "Fasthand-Regular.ttf" => font_bytes })
      Typst(body: main, dependencies: { "template.typ" => template, "icon.svg" => icon }, fonts: { "Fasthand-Regular.ttf" => font_bytes }).compile(:pdf)
      Typst(body: main).with_dependencies({ "template.typ" => template, "icon.svg" => icon }).with_fonts({ "Fasthand-Regular.ttf" => font_bytes }).compile(:pdf)
    }
  end

  def test_from_zip_main_typ
    assert{
      Typst::Svg::from_zip("main.typ.zip")
      Typst(zip: "main.typ.zip").compile(:svg)
    }
  end

  def test_from_zip_hello_typ
    assert{
      Typst::Svg::from_zip("hello.typ.zip")
      Typst(zip: "hello.typ.zip").compile(:svg)
    }
  end

  def test_from_zip_with_fonts
    assert{
      Typst::Svg::from_zip("with_fonts.zip")
      Typst(zip: "with_fonts.zip").compile(:svg)
    }
  end

  def test_from_zip_no_fonts
    assert{
      Typst::Svg::from_zip("no_fonts.zip")
      Typst(zip: "no_fonts.zip").compile(:svg)
    }
  end

  def test_from_zip_named_main
    assert{
      Typst::Svg::from_zip("two.zip", "hello.typ")
      Typst(zip: "two.zip", main_file: "hello.typ").compile(:svg)
    }
  end

  def test_from_zip_no_main
    assert_raise{
      Typst::Svg::from_zip("two.zip")
      Typst(zip: "two.zip").compile(:svg)
    }
  end

  def test_sys_inputs
    assert {
      require "json"
      require "hexapdf"
      require "#{File.dirname(__FILE__)}/text_processor.rb"

      # Pass values into a typst template using sys_inputs
      sys_inputs_example = %{
      #let persons = json(bytes(sys.inputs.persons))

      #for person in persons [
        #person.name is #person.age years old.\\
      ]
      }
      t = Typst::Pdf.from_s(sys_inputs_example, sys_inputs: { "persons" => [{"name": "John", "age": 35}, {"name": "Xoliswa", "age": 45}].to_json })

      reader = HexaPDF::Document.new(io: StringIO.open(t.document))
      processor = GetTextProcessor.new
      reader.pages.each{ |page| page.process_contents(processor) }
      processor.string == "John is 35 years old.\nXoliswa is 45 years old.\n"

      t2 = Typst(body: sys_inputs_example).with_inputs({ "persons" => [{"name": "John", "age": 35}, {"name": "Xoliswa", "age": 45}].to_json }).compile(:pdf)

      reader2 = HexaPDF::Document.new(io: StringIO.open(t2.document))
      processor2 = GetTextProcessor.new
      reader2.pages.each{ |page| page.process_contents(processor2) }
      processor2.string == "John is 35 years old.\nXoliswa is 45 years old.\n"
    }
  end
end