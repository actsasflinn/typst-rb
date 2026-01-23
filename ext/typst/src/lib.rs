use std::path::PathBuf;

use magnus::{function, prelude::*, Error, Ruby};

use std::collections::HashMap;
use query::{query as typst_query, QueryCommand, SerializationFormat};
use typst::foundations::{Dict, Value};
use typst_library::Feature;
use typst_pdf::PdfStandard;
use world::SystemWorld;

mod compiler;
mod download;
mod query;
mod world;

fn to_html(
    ruby: &Ruby,
    input: PathBuf,
    root: Option<PathBuf>,
    font_paths: Vec<PathBuf>,
    resource_path: PathBuf,
    ignore_system_fonts: bool,
    sys_inputs: HashMap<String, String>,
) -> Result<Vec<Vec<u8>>, Error> {
    let input = input.canonicalize()
        .map_err(|err| magnus::Error::new(ruby.exception_arg_error(), err.to_string()))?;

    let root = if let Some(root) = root {
        root.canonicalize()
            .map_err(|err| magnus::Error::new(ruby.exception_arg_error(), err.to_string()))?
    } else if let Some(dir) = input.parent() {
        dir.into()
    } else {
        PathBuf::new()
    };

    let resource_path = resource_path.canonicalize()
        .map_err(|err| magnus::Error::new(ruby.exception_arg_error(), err.to_string()))?;

    let mut default_fonts = Vec::new();
    for entry in walkdir::WalkDir::new(resource_path.join("fonts")) {
        let path = entry
            .map_err(|err| magnus::Error::new(ruby.exception_arg_error(), err.to_string()))?
            .into_path();
        let Some(extension) = path.extension() else {
            continue;
        };
        if extension == "ttf" || extension == "otf" {
            default_fonts.push(path);
        }
    }

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
        .build()
        .map_err(|msg| magnus::Error::new(ruby.exception_arg_error(), msg.to_string()))?;

    let bytes = world
        .compile(Some("html"), None, &Vec::new())
        .map_err(|msg| magnus::Error::new(ruby.exception_arg_error(), msg.to_string()))?;

    Ok(bytes)
}

fn to_svg(
    ruby: &Ruby,
    input: PathBuf,
    root: Option<PathBuf>,
    font_paths: Vec<PathBuf>,
    resource_path: PathBuf,
    ignore_system_fonts: bool,
    sys_inputs: HashMap<String, String>,
) -> Result<Vec<Vec<u8>>, Error> {
    let input = input.canonicalize()
        .map_err(|err| magnus::Error::new(ruby.exception_arg_error(), err.to_string()))?;

    let root = if let Some(root) = root {
        root.canonicalize()
            .map_err(|err| magnus::Error::new(ruby.exception_arg_error(), err.to_string()))?
    } else if let Some(dir) = input.parent() {
        dir.into()
    } else {
        PathBuf::new()
    };

    let resource_path = resource_path.canonicalize()
        .map_err(|err| magnus::Error::new(ruby.exception_arg_error(), err.to_string()))?;

    let mut default_fonts = Vec::new();
    for entry in walkdir::WalkDir::new(resource_path.join("fonts")) {
        let path = entry
            .map_err(|err| magnus::Error::new(ruby.exception_arg_error(), err.to_string()))?
            .into_path();
        let Some(extension) = path.extension() else {
            continue;
        };
        if extension == "ttf" || extension == "otf" {
            default_fonts.push(path);
        }
    }

    let mut world = SystemWorld::builder(root, input)
        .inputs(Dict::from_iter(
            sys_inputs
                .into_iter()
                .map(|(k, v)| (k.into(), Value::Str(v.into()))),
        ))
        .font_paths(font_paths)
        .ignore_system_fonts(ignore_system_fonts)
        .build()
        .map_err(|msg| magnus::Error::new(ruby.exception_arg_error(), msg.to_string()))?;

    let svg_bytes = world
        .compile(Some("svg"), None, &Vec::new())
        .map_err(|msg| magnus::Error::new(ruby.exception_arg_error(), msg.to_string()))?;

    Ok(svg_bytes)
}

fn to_png(
    ruby: &Ruby,
    input: PathBuf,
    root: Option<PathBuf>,
    font_paths: Vec<PathBuf>,
    resource_path: PathBuf,
    ignore_system_fonts: bool,
    sys_inputs: HashMap<String, String>,
    ppi: Option<f32>,
) -> Result<Vec<Vec<u8>>, Error> {
    let input = input.canonicalize()
        .map_err(|err| magnus::Error::new(ruby.exception_arg_error(), err.to_string()))?;

    let root = if let Some(root) = root {
        root.canonicalize()
            .map_err(|err| magnus::Error::new(ruby.exception_arg_error(), err.to_string()))?
    } else if let Some(dir) = input.parent() {
        dir.into()
    } else {
        PathBuf::new()
    };

    let resource_path = resource_path.canonicalize()
        .map_err(|err| magnus::Error::new(ruby.exception_arg_error(), err.to_string()))?;

    let mut default_fonts = Vec::new();
    for entry in walkdir::WalkDir::new(resource_path.join("fonts")) {
        let path = entry
            .map_err(|err| magnus::Error::new(ruby.exception_arg_error(), err.to_string()))?
            .into_path();
        let Some(extension) = path.extension() else {
            continue;
        };
        if extension == "ttf" || extension == "otf" {
            default_fonts.push(path);
        }
    }

    let mut world = SystemWorld::builder(root, input)
        .inputs(Dict::from_iter(
            sys_inputs
                .into_iter()
                .map(|(k, v)| (k.into(), Value::Str(v.into()))),
        ))
        .font_paths(font_paths)
        .ignore_system_fonts(ignore_system_fonts)
        .build()
        .map_err(|msg| magnus::Error::new(ruby.exception_arg_error(), msg.to_string()))?;

    let bytes = world
        .compile(Some("png"), ppi, &Vec::new())
        .map_err(|msg| magnus::Error::new(ruby.exception_arg_error(), msg.to_string()))?;

    Ok(bytes)
}

fn to_pdf(
    ruby: &Ruby,
    input: PathBuf,
    root: Option<PathBuf>,
    font_paths: Vec<PathBuf>,
    resource_path: PathBuf,
    ignore_system_fonts: bool,
    sys_inputs: HashMap<String, String>,
    pdf_standards: Vec<String>,
) -> Result<Vec<Vec<u8>>, Error> {
    let input = input.canonicalize()
        .map_err(|err| magnus::Error::new(ruby.exception_arg_error(), err.to_string()))?;

    let root = if let Some(root) = root {
        root.canonicalize()
            .map_err(|err| magnus::Error::new(ruby.exception_arg_error(), err.to_string()))?
    } else if let Some(dir) = input.parent() {
        dir.into()
    } else {
        PathBuf::new()
    };

    let resource_path = resource_path.canonicalize()
        .map_err(|err| magnus::Error::new(ruby.exception_arg_error(), err.to_string()))?;

    let mut default_fonts = Vec::new();
    for entry in walkdir::WalkDir::new(resource_path.join("fonts")) {
        let path = entry
            .map_err(|err| magnus::Error::new(ruby.exception_arg_error(), err.to_string()))?
            .into_path();
        let Some(extension) = path.extension() else {
            continue;
        };
        if extension == "ttf" || extension == "otf" {
            default_fonts.push(path);
        }
    }

    let mut world = SystemWorld::builder(root, input)
        .inputs(Dict::from_iter(
            sys_inputs
                .into_iter()
                .map(|(k, v)| (k.into(), Value::Str(v.into()))),
        ))
        .font_paths(font_paths)
        .ignore_system_fonts(ignore_system_fonts)
        .build()
        .map_err(|msg| magnus::Error::new(ruby.exception_arg_error(), msg.to_string()))?;

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
            _ => return Err(magnus::Error::new(ruby.exception_arg_error(), "Unknown PdfStandard")),
        }
    }

    let pdf_bytes = world
        .compile(Some("pdf"), None, &pdf_standards_vec)
        .map_err(|msg| magnus::Error::new(ruby.exception_arg_error(), msg.to_string()))?;

    Ok(pdf_bytes)
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
    resource_path: PathBuf,
    ignore_system_fonts: bool,
    sys_inputs: HashMap<String, String>,
) -> Result<String, Error> {
    let format = match format.unwrap().to_ascii_lowercase().as_str() {
        "json" => SerializationFormat::Json,
        "yaml" => SerializationFormat::Yaml,
        _ => return Err(magnus::Error::new(ruby.exception_arg_error(), "unsupported serialization format"))?,
    };

    let input = input.canonicalize()
        .map_err(|err| magnus::Error::new(ruby.exception_arg_error(), err.to_string()))?;

    let root = if let Some(root) = root {
        root.canonicalize()
            .map_err(|err| magnus::Error::new(ruby.exception_arg_error(), err.to_string()))?
    } else if let Some(dir) = input.parent() {
        dir.into()
    } else {
        PathBuf::new()
    };

    let resource_path = resource_path.canonicalize()
        .map_err(|err| magnus::Error::new(ruby.exception_arg_error(), err.to_string()))?;

    let mut default_fonts = Vec::new();
    for entry in walkdir::WalkDir::new(resource_path.join("fonts")) {
        let path = entry
            .map_err(|err| magnus::Error::new(ruby.exception_arg_error(), err.to_string()))?
            .into_path();
        let Some(extension) = path.extension() else {
            continue;
        };
        if extension == "ttf" || extension == "otf" {
            default_fonts.push(path);
        }
    }

    let mut world = SystemWorld::builder(root, input)
    .inputs(Dict::from_iter(
        sys_inputs
            .into_iter()
            .map(|(k, v)| (k.into(), Value::Str(v.into()))),
    ))
    .font_paths(font_paths)
    .ignore_system_fonts(ignore_system_fonts)
    .build()
    .map_err(|msg| magnus::Error::new(ruby.exception_arg_error(), msg.to_string()))?;

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
        Err(msg) => Err(magnus::Error::new(ruby.exception_arg_error(), msg.to_string())),
    }
}

fn clear_cache(_ruby: &Ruby, max_age: usize) {
    comemo::evict(max_age);
}

#[magnus::init]
fn init(ruby: &Ruby) -> Result<(), Error> {
    env_logger::init();

    let module = ruby.define_module("Typst")?;
    module.define_singleton_method("_to_pdf", function!(to_pdf, 7))?;
    module.define_singleton_method("_to_svg", function!(to_svg, 6))?;
    module.define_singleton_method("_to_png", function!(to_png, 7))?;
    module.define_singleton_method("_to_html", function!(to_html, 6))?;
    module.define_singleton_method("_query", function!(query, 10))?;
    module.define_singleton_method("_clear_cache", function!(clear_cache, 1))?;
    Ok(())
}