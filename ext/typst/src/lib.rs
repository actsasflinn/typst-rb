use magnus::{function, method, prelude::*, Error, Ruby, scan_args::scan_args};

use world::VirtualWorld;
use formats::pdf::*;
use formats::html::*;
use formats::svg::*;
use formats::png::*;

mod download;
mod formats;
mod nogvl;
mod query;
mod world;

fn clear_compiler_cache(args: &[magnus::Value]) {
    let args = scan_args::<(), _, (), (), (), ()>(args).unwrap();
    let (max_age,): (Option<usize>,) = args.optional;

    comemo::evict(max_age.unwrap_or(0));
}

#[magnus::init]
fn init(ruby: &Ruby) -> Result<(), Error> {
    env_logger::init();

    let typst = ruby.define_module("Typst")?;
    typst.define_singleton_method("clear_cache", function!(clear_compiler_cache, -1))?;
    typst.define_singleton_method("add_font", function!(world::add_font, 1))?;
    typst.define_singleton_method("add_fonts", function!(world::add_fonts, 1))?;
    typst.define_singleton_method("prime_font_cache", function!(world::prime_font_cache, 2))?;
    typst.define_singleton_method("clear_font_cache", function!(world::clear_font_cache, 0))?;
    typst.define_singleton_method("add_file", function!(world::add_file, 2))?;
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

    let document = typst.define_class("PdfDocument", ruby.class_object())?;
    document.define_singleton_method("new", function!(PdfDocument::new, 2))?;
    document.define_method("document", method!(PdfDocument::document, 0))?;
    document.define_method("bytes", method!(PdfDocument::bytes, 0))?;
    document.define_method("warnings", method!(PdfDocument::warnings, 0))?;
    document.define_method("warnings?", method!(PdfDocument::has_warnings, 0))?;
    document.define_method("write", method!(PdfDocument::write, 1))?;

    let html_document = typst.define_class("HtmlExperimentalDocument", ruby.class_object())?;
    html_document.define_singleton_method("new", function!(HtmlExperimentalDocument::new, 2))?;
    html_document.define_method("document", method!(HtmlExperimentalDocument::document, 0))?;
    html_document.define_method("bytes", method!(HtmlExperimentalDocument::bytes, 0))?;
    html_document.define_method("warnings", method!(HtmlExperimentalDocument::warnings, 0))?;
    html_document.define_method("warnings?", method!(HtmlExperimentalDocument::has_warnings, 0))?;
    html_document.define_method("write", method!(HtmlExperimentalDocument::write, 1))?;

    let svg_document = typst.define_class("SvgDocument", ruby.class_object())?;
    svg_document.define_singleton_method("new", function!(SvgDocument::new, 2))?;
    svg_document.define_method("bytes", method!(SvgDocument::bytes, 0))?;
    svg_document.define_method("pages", method!(SvgDocument::pages, 0))?;
    svg_document.define_method("warnings", method!(SvgDocument::warnings, 0))?;
    svg_document.define_method("warnings?", method!(SvgDocument::has_warnings, 0))?;
    svg_document.define_method("write", method!(SvgDocument::write, 1))?;

    let png_document = typst.define_class("PngDocument", ruby.class_object())?;
    png_document.define_singleton_method("new", function!(PngDocument::new, 2))?;
    png_document.define_method("bytes", method!(PngDocument::bytes, 0))?;
    png_document.define_method("pages", method!(PngDocument::pages, 0))?;
    png_document.define_method("warnings", method!(PngDocument::warnings, 0))?;
    png_document.define_method("warnings?", method!(PngDocument::has_warnings, 0))?;
    png_document.define_method("write", method!(PngDocument::write, 1))?;

    Ok(())
}