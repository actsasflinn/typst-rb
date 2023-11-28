require "test/unit"
require "base64"
require_relative "../lib/typst"

class TypstTest < Test::Unit::TestCase

  def test_pdf
    assert {
      Typst::Pdf.new("test.typ")
    }
  end

  def test_pdf_assets
    assert {
      t = Typst::Pdf.new("template_with_font_and_icon/main.typ", root: "template_with_font_and_icon")
    }
  end

  def test_pdf_without_font
    assert {
      Typst::Pdf.new("template_with_font_and_icon/main.typ", root: "template_with_font_and_icon").document.include?("Fasthand") == false
    }
  end

  def test_pdf_font
    assert {
      Typst::Pdf.new("template_with_font_and_icon/main.typ", root: "template_with_font_and_icon", font_paths: ["./fonts/Fasthand/Release/ttf"]).document.include?("Fasthand")
    }
  end

  def test_svg
    assert {
      t = Typst::Svg.new("test.typ")
    }
  end

  def test_html
    assert {
      t = Typst::Html.new("test.typ")
    }
  end

  def test_from_s
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

    icon = File.read("template_with_font_and_icon/monkey.svg")
    font_bytes = File.read("./fonts/Fasthand/Release/ttf/Fasthand-Regular.ttf")

    t = Typst::Pdf.from_s(main, dependencies: { "template.typ" => template, "icon.svg" => icon }, fonts: { "Fasthand-Regular.ttf" => font_bytes })
  end
end