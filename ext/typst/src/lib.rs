use std::path::PathBuf;

use magnus::{function, prelude::*, Error, Ruby};

use std::collections::HashMap;
use query::{query as typst_query, QueryCommand, SerializationFormat};
use typst::foundations::{Dict, Value};
use typst_library::Feature;
use typst_pdf::PdfStandard;
use nogvl::maybe_without_gvl;
use world::SystemWorld;

mod compiler;
mod download;
mod nogvl;
mod query;
mod world;

fn to_html(
    ruby: &Ruby,
    input: PathBuf,
    root: Option<PathBuf>,
    font_paths: Vec<PathBuf>,
    ignore_system_fonts: bool,
    ignore_embedded_fonts: bool,
    render_bleed: bool,
    pretty: bool,
    sys_inputs: HashMap<String, String>,
    release_gvl: bool,
) -> Result<(Vec<Vec<u8>>, Vec<String>), Error> {
    maybe_without_gvl(release_gvl, move || -> Result<(Vec<Vec<u8>>, Vec<String>), String> {
        let input = input.canonicalize()
            .map_err(|err| err.to_string())?;

        let root = if let Some(root) = root {
            root.canonicalize()
                .map_err(|err| err.to_string())?
        } else if let Some(dir) = input.parent() {
            dir.into()
        } else {
            PathBuf::new()
        };

        let mut features = Vec::new();
        features.push(Feature::Html);

        let feat = features.iter()
        .map(|&feature|
            match feature {
                Feature::Html => typst::Feature::Html,
                _ => typst::Feature::Html // TODO: fix this hack
            }
        )
        .collect();

        let mut world = SystemWorld::builder(root, input)
            .inputs(Dict::from_iter(
                sys_inputs
                    .into_iter()
                    .map(|(k, v)| (k.into(), Value::Str(v.into()))),
            ))
            .features(feat)
            .font_paths(font_paths)
            .ignore_system_fonts(ignore_system_fonts)
            .ignore_embedded_fonts(ignore_embedded_fonts)
            .build()
            .map_err(|msg| msg.to_string())?;

        let compiled = world
            .compile(Some("html"), None, &Vec::new(), pretty, render_bleed)
            .map_err(|msg| msg.to_string())?;

        Ok(compiled)
    })
    .map_err(|msg| magnus::Error::new(ruby.exception_arg_error(), msg))
}

fn to_svg(
    ruby: &Ruby,
    input: PathBuf,
    root: Option<PathBuf>,
    font_paths: Vec<PathBuf>,
    ignore_system_fonts: bool,
    ignore_embedded_fonts: bool,
    render_bleed: bool,
    pretty: bool,
    sys_inputs: HashMap<String, String>,
    release_gvl: bool,
) -> Result<(Vec<Vec<u8>>, Vec<String>), Error> {
    maybe_without_gvl(release_gvl, move || -> Result<(Vec<Vec<u8>>, Vec<String>), String> {
        let input = input.canonicalize()
            .map_err(|err| err.to_string())?;

        let root = if let Some(root) = root {
            root.canonicalize()
                .map_err(|err| err.to_string())?
        } else if let Some(dir) = input.parent() {
            dir.into()
        } else {
            PathBuf::new()
        };

        let mut world = SystemWorld::builder(root, input)
            .inputs(Dict::from_iter(
                sys_inputs
                    .into_iter()
                    .map(|(k, v)| (k.into(), Value::Str(v.into()))),
            ))
            .font_paths(font_paths)
            .ignore_system_fonts(ignore_system_fonts)
            .ignore_embedded_fonts(ignore_embedded_fonts)
            .build()
            .map_err(|msg| msg.to_string())?;

        let compiled = world
            .compile(Some("svg"), None, &Vec::new(), pretty, render_bleed)
            .map_err(|msg| msg.to_string())?;

        Ok(compiled)
    })
    .map_err(|msg| magnus::Error::new(ruby.exception_arg_error(), msg))
}

fn to_png(
    ruby: &Ruby,
    input: PathBuf,
    root: Option<PathBuf>,
    font_paths: Vec<PathBuf>,
    ignore_system_fonts: bool,
    ignore_embedded_fonts: bool,
    render_bleed: bool,
    sys_inputs: HashMap<String, String>,
    ppi: Option<f32>,
    release_gvl: bool,
) -> Result<(Vec<Vec<u8>>, Vec<String>), Error> {
    maybe_without_gvl(release_gvl, move || -> Result<(Vec<Vec<u8>>, Vec<String>), String> {
        let input = input.canonicalize()
            .map_err(|err| err.to_string())?;

        let root = if let Some(root) = root {
            root.canonicalize()
                .map_err(|err| err.to_string())?
        } else if let Some(dir) = input.parent() {
            dir.into()
        } else {
            PathBuf::new()
        };

        let mut world = SystemWorld::builder(root, input)
            .inputs(Dict::from_iter(
                sys_inputs
                    .into_iter()
                    .map(|(k, v)| (k.into(), Value::Str(v.into()))),
            ))
            .font_paths(font_paths)
            .ignore_system_fonts(ignore_system_fonts)
            .ignore_embedded_fonts(ignore_embedded_fonts)
            .build()
            .map_err(|msg| msg.to_string())?;

        let compiled = world
            .compile(Some("png"), ppi, &Vec::new(), false, render_bleed)
            .map_err(|msg| msg.to_string())?;

        Ok(compiled)
    })
    .map_err(|msg| magnus::Error::new(ruby.exception_arg_error(), msg))
}

fn to_pdf(
    ruby: &Ruby,
    input: PathBuf,
    root: Option<PathBuf>,
    font_paths: Vec<PathBuf>,
    ignore_system_fonts: bool,
    ignore_embedded_fonts: bool,
    pretty: bool,
    sys_inputs: HashMap<String, String>,
    pdf_standards: Vec<String>,
    release_gvl: bool,
) -> Result<(Vec<Vec<u8>>, Vec<String>), Error> {
    maybe_without_gvl(release_gvl, move || -> Result<(Vec<Vec<u8>>, Vec<String>), String> {
        let input = input.canonicalize()
            .map_err(|err| err.to_string())?;

        let root = if let Some(root) = root {
            root.canonicalize()
                .map_err(|err| err.to_string())?
        } else if let Some(dir) = input.parent() {
            dir.into()
        } else {
            PathBuf::new()
        };

        let mut world = SystemWorld::builder(root, input)
            .inputs(Dict::from_iter(
                sys_inputs
                    .into_iter()
                    .map(|(k, v)| (k.into(), Value::Str(v.into()))),
            ))
            .font_paths(font_paths)
            .ignore_system_fonts(ignore_system_fonts)
            .ignore_embedded_fonts(ignore_embedded_fonts)
            .build()
            .map_err(|msg| msg.to_string())?;

        let pdf_standards_lookup: HashMap::<&str, PdfStandard> = HashMap::from([
            ("1.4", PdfStandard::V_1_4),
            ("1.5", PdfStandard::V_1_5),
            ("1.6", PdfStandard::V_1_6),
            ("1.7", PdfStandard::V_1_7),
            ("2.0", PdfStandard::V_2_0),
            ("a-1a", PdfStandard::A_1a),
            ("a-1b", PdfStandard::A_1b),
            ("a-2a", PdfStandard::A_2a),
            ("a-2b", PdfStandard::A_2b),
            ("a-2u", PdfStandard::A_2u),
            ("a-3a", PdfStandard::A_3a),
            ("a-3b", PdfStandard::A_3b),
            ("a-3u", PdfStandard::A_3u),
            ("a-4", PdfStandard::A_4),
            ("a-4e", PdfStandard::A_4e),
            ("a-4f", PdfStandard::A_4f),
            ("ua-1", PdfStandard::Ua_1),
        ]);

        let mut pdf_standards_vec = Vec::<PdfStandard>::new();
        for pdf_standard in pdf_standards.iter() {
            let result = pdf_standards_lookup.get(pdf_standard.as_str());
            match result {
                Some(value) => pdf_standards_vec.push(*value),
                _ => return Err("Unknown PdfStandard".to_string()),
            }
        }

        let compiled = world
            .compile(Some("pdf"), None, &pdf_standards_vec, pretty, true)
            .map_err(|msg| msg.to_string())?;

        Ok(compiled)
    })
    .map_err(|msg| magnus::Error::new(ruby.exception_arg_error(), msg))
}

fn query(
    ruby: &Ruby,
    selector: String,
    field: Option<String>,
    one: bool,
    format: Option<String>,
    input: PathBuf,
    root: Option<PathBuf>,
    font_paths: Vec<PathBuf>,
    ignore_system_fonts: bool,
    ignore_embedded_fonts: bool,
    sys_inputs: HashMap<String, String>,
    release_gvl: bool,
) -> Result<String, Error> {
    maybe_without_gvl(release_gvl, move || -> Result<String, String> {
        let format = match format.unwrap().to_ascii_lowercase().as_str() {
            "json" => SerializationFormat::Json,
            "yaml" => SerializationFormat::Yaml,
            _ => return Err("unsupported serialization format".to_string()),
        };

        let input = input.canonicalize()
            .map_err(|err| err.to_string())?;

        let root = if let Some(root) = root {
            root.canonicalize()
                .map_err(|err| err.to_string())?
        } else if let Some(dir) = input.parent() {
            dir.into()
        } else {
            PathBuf::new()
        };

        let mut world = SystemWorld::builder(root, input)
        .inputs(Dict::from_iter(
            sys_inputs
                .into_iter()
                .map(|(k, v)| (k.into(), Value::Str(v.into()))),
        ))
        .font_paths(font_paths)
        .ignore_system_fonts(ignore_system_fonts)
        .ignore_embedded_fonts(ignore_embedded_fonts)
        .build()
        .map_err(|msg| msg.to_string())?;

        let result = typst_query(
            &mut world,
            &QueryCommand {
                selector: selector.into(),
                field: field.map(Into::into),
                one,
                format,
            },
        );

        match result {
            Ok(data) => Ok(data),
            Err(msg) => Err(msg.to_string()),
        }
    })
    .map_err(|msg| magnus::Error::new(ruby.exception_arg_error(), msg))
}

fn clear_cache(_ruby: &Ruby, max_age: usize) {
    comemo::evict(max_age);
}

fn clear_font_cache(_ruby: &Ruby) {
    world::clear_font_cache();
}

#[magnus::init]
fn init(ruby: &Ruby) -> Result<(), Error> {
    env_logger::init();

    let module = ruby.define_module("Typst")?;
    module.define_singleton_method("_to_pdf", function!(to_pdf, 9))?;
    module.define_singleton_method("_to_svg", function!(to_svg, 9))?;
    module.define_singleton_method("_to_png", function!(to_png, 9))?;
    module.define_singleton_method("_to_html", function!(to_html, 9))?;
    module.define_singleton_method("_query", function!(query, 11))?;
    module.define_singleton_method("_clear_cache", function!(clear_cache, 1))?;
    module.define_singleton_method("_clear_font_cache", function!(clear_font_cache, 0))?;
    Ok(())
}