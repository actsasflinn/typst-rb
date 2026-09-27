use magnus::{function, method, prelude::*, Error, Ruby};

use world::VirtualWorld;
use formats::pdf::*;
use formats::html::*;
use formats::svg::*;
use formats::png::*;

mod world;
mod nogvl;
mod formats;
mod query;

#[magnus::init]
fn init(ruby: &Ruby) -> Result<(), Error> {
    env_logger::init();

    let typst = ruby.define_module("Typst")?;
    typst.define_singleton_method("add_font", function!(world::add_font, 1))?;
    typst.define_singleton_method("add_file", function!(world::add_file, 2))?;
    typst.define_singleton_method("clear_font_cache", function!(world::clear_font_cache, 0))?;
    typst.define_singleton_method("clear_file_cache", function!(world::clear_file_cache, 0))?;

    let virtual_world = typst.define_class("VirtualWorld", ruby.class_object())?;
    virtual_world.define_singleton_method("new", function!(VirtualWorld::new_ruby, -1))?;
    virtual_world.define_method("to_pdf", method!(VirtualWorld::to_pdf_ruby, -1))?;
    virtual_world.define_method("to_html", method!(VirtualWorld::to_html_ruby, -1))?;
    virtual_world.define_method("to_svg", method!(VirtualWorld::to_svg_ruby, -1))?;
    virtual_world.define_method("to_png", method!(VirtualWorld::to_png_ruby, -1))?;
    virtual_world.define_method("info", method!(VirtualWorld::info, 0))?;
    virtual_world.define_method("with_inputs", method!(VirtualWorld::with_inputs, 1))?;
    virtual_world.define_method("query", method!(VirtualWorld::query, 4))?;

    let pdf = typst.define_class("Pdf", ruby.class_object())?;
    pdf.define_singleton_method("new", function!(Pdf::new_ruby, -1))?;
    pdf.define_method("compiled", method!(Pdf::compiled, 0))?;

    let document = typst.define_class("PdfDocument", ruby.class_object())?;
    document.define_singleton_method("new", function!(PdfDocument::new, 2))?;
    document.define_method("bytes", method!(PdfDocument::bytes, 0))?;
    document.define_method("warnings", method!(PdfDocument::warnings, 0))?;
    document.define_method("warnings?", method!(PdfDocument::has_warnings, 0))?;
    document.define_method("write", method!(PdfDocument::write, 1))?;

    let html = typst.define_class("Html", ruby.class_object())?;
    html.define_singleton_method("new", function!(Html::new_ruby, -1))?;
    html.define_method("compiled", method!(Html::compiled, 0))?;

    let html_document = typst.define_class("HtmlExperimentalDocument", ruby.class_object())?;
    html_document.define_singleton_method("new", function!(HtmlExperimentalDocument::new, 2))?;
    html_document.define_method("document", method!(HtmlExperimentalDocument::document, 0))?;
    html_document.define_method("bytes", method!(HtmlExperimentalDocument::bytes, 0))?;
    html_document.define_method("warnings", method!(HtmlExperimentalDocument::warnings, 0))?;
    html_document.define_method("warnings?", method!(HtmlExperimentalDocument::has_warnings, 0))?;
    html_document.define_method("write", method!(HtmlExperimentalDocument::write, 1))?;

    let svg = typst.define_class("Svg", ruby.class_object())?;
    svg.define_singleton_method("new", function!(Svg::new_ruby, -1))?;
    svg.define_method("compiled", method!(Svg::compiled, 0))?;

    let svg_document = typst.define_class("SvgDocument", ruby.class_object())?;
    svg_document.define_singleton_method("new", function!(SvgDocument::new, 2))?;
    svg_document.define_method("bytes", method!(SvgDocument::bytes, 0))?;
    svg_document.define_method("pages", method!(SvgDocument::pages, 0))?;
    svg_document.define_method("warnings", method!(SvgDocument::warnings, 0))?;
    svg_document.define_method("warnings?", method!(SvgDocument::has_warnings, 0))?;
    svg_document.define_method("write", method!(SvgDocument::write, 1))?;

    let png = typst.define_class("Png", ruby.class_object())?;
    png.define_singleton_method("new", function!(Png::new_ruby, -1))?;
    png.define_method("compiled", method!(Png::compiled, 0))?;

    let png_document = typst.define_class("PngDocument", ruby.class_object())?;
    png_document.define_singleton_method("new", function!(PngDocument::new, 2))?;
    png_document.define_method("bytes", method!(PngDocument::bytes, 0))?;
    png_document.define_method("pages", method!(PngDocument::pages, 0))?;
    png_document.define_method("warnings", method!(PngDocument::warnings, 0))?;
    png_document.define_method("warnings?", method!(PngDocument::has_warnings, 0))?;
    png_document.define_method("write", method!(PngDocument::write, 1))?;

    Ok(())
}