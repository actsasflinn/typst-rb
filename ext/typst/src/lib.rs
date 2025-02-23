use std::path::PathBuf;

use magnus::{define_module, function, exception, Error }; //, IntoValue};
use magnus::{prelude::*};

use std::collections::HashMap;
use typst::foundations::{Dict, Value};
use typst_library::Feature;
use world::SystemWorld;

mod compiler;
mod download;
mod query;
mod world;

fn to_html(
    input: PathBuf,
    root: Option<PathBuf>,
    font_paths: Vec<PathBuf>,
    resource_path: PathBuf,
    ignore_system_fonts: bool,
    sys_inputs: HashMap<String, String>,
) -> Result<Vec<Vec<u8>>, Error> {
    let input = input.canonicalize()
        .map_err(|err| magnus::Error::new(exception::arg_error(), err.to_string()))?;

    let root = if let Some(root) = root {
        root.canonicalize()
            .map_err(|err| magnus::Error::new(exception::arg_error(), err.to_string()))?
    } else if let Some(dir) = input.parent() {
        dir.into()
    } else {
        PathBuf::new()
    };

    let resource_path = resource_path.canonicalize()
        .map_err(|err| magnus::Error::new(exception::arg_error(), err.to_string()))?;

    let mut default_fonts = Vec::new();
    for entry in walkdir::WalkDir::new(resource_path.join("fonts")) {
        let path = entry
            .map_err(|err| magnus::Error::new(exception::arg_error(), err.to_string()))?
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
        .map_err(|msg| magnus::Error::new(exception::arg_error(), msg.to_string()))?;

    let bytes = world
        .compile(Some("html"), None, &Vec::new())
        .map_err(|msg| magnus::Error::new(exception::arg_error(), msg.to_string()))?;

    Ok(bytes)
}

fn to_svg(
    input: PathBuf,
    root: Option<PathBuf>,
    font_paths: Vec<PathBuf>,
    resource_path: PathBuf,
    ignore_system_fonts: bool,
    sys_inputs: HashMap<String, String>,
) -> Result<Vec<Vec<u8>>, Error> {
    let input = input.canonicalize()
        .map_err(|err| magnus::Error::new(exception::arg_error(), err.to_string()))?;

    let root = if let Some(root) = root {
        root.canonicalize()
            .map_err(|err| magnus::Error::new(exception::arg_error(), err.to_string()))?
    } else if let Some(dir) = input.parent() {
        dir.into()
    } else {
        PathBuf::new()
    };

    let resource_path = resource_path.canonicalize()
        .map_err(|err| magnus::Error::new(exception::arg_error(), err.to_string()))?;

    let mut default_fonts = Vec::new();
    for entry in walkdir::WalkDir::new(resource_path.join("fonts")) {
        let path = entry
            .map_err(|err| magnus::Error::new(exception::arg_error(), err.to_string()))?
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
        .map_err(|msg| magnus::Error::new(exception::arg_error(), msg.to_string()))?;

    let svg_bytes = world
        .compile(Some("svg"), None, &Vec::new())
        .map_err(|msg| magnus::Error::new(exception::arg_error(), msg.to_string()))?;

    Ok(svg_bytes)
}

fn to_png(
    input: PathBuf,
    root: Option<PathBuf>,
    font_paths: Vec<PathBuf>,
    resource_path: PathBuf,
    ignore_system_fonts: bool,
    sys_inputs: HashMap<String, String>,
) -> Result<Vec<Vec<u8>>, Error> {
    let input = input.canonicalize()
        .map_err(|err| magnus::Error::new(exception::arg_error(), err.to_string()))?;

    let root = if let Some(root) = root {
        root.canonicalize()
            .map_err(|err| magnus::Error::new(exception::arg_error(), err.to_string()))?
    } else if let Some(dir) = input.parent() {
        dir.into()
    } else {
        PathBuf::new()
    };

    let resource_path = resource_path.canonicalize()
        .map_err(|err| magnus::Error::new(exception::arg_error(), err.to_string()))?;

    let mut default_fonts = Vec::new();
    for entry in walkdir::WalkDir::new(resource_path.join("fonts")) {
        let path = entry
            .map_err(|err| magnus::Error::new(exception::arg_error(), err.to_string()))?
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
        .map_err(|msg| magnus::Error::new(exception::arg_error(), msg.to_string()))?;

    let bytes = world
        .compile(Some("png"), None, &Vec::new())
        .map_err(|msg| magnus::Error::new(exception::arg_error(), msg.to_string()))?;

    Ok(bytes)
}

fn to_pdf(
    input: PathBuf,
    root: Option<PathBuf>,
    font_paths: Vec<PathBuf>,
    resource_path: PathBuf,
    ignore_system_fonts: bool,
    sys_inputs: HashMap<String, String>,
) -> Result<Vec<Vec<u8>>, Error> {
    let input = input.canonicalize()
        .map_err(|err| magnus::Error::new(exception::arg_error(), err.to_string()))?;

    let root = if let Some(root) = root {
        root.canonicalize()
            .map_err(|err| magnus::Error::new(exception::arg_error(), err.to_string()))?
    } else if let Some(dir) = input.parent() {
        dir.into()
    } else {
        PathBuf::new()
    };

    let resource_path = resource_path.canonicalize()
        .map_err(|err| magnus::Error::new(exception::arg_error(), err.to_string()))?;

    let mut default_fonts = Vec::new();
    for entry in walkdir::WalkDir::new(resource_path.join("fonts")) {
        let path = entry
            .map_err(|err| magnus::Error::new(exception::arg_error(), err.to_string()))?
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
        .map_err(|msg| magnus::Error::new(exception::arg_error(), msg.to_string()))?;

    let pdf_bytes = world
        .compile(Some("pdf"), None, &Vec::new())
        .map_err(|msg| magnus::Error::new(exception::arg_error(), msg.to_string()))?;

    Ok(pdf_bytes)
}

#[magnus::init]
fn init() -> Result<(), Error> {
    env_logger::init();

    let module = define_module("Typst")?;
    module.define_singleton_method("_to_pdf", function!(to_pdf, 6))?;
    module.define_singleton_method("_to_svg", function!(to_svg, 6))?;
    module.define_singleton_method("_to_png", function!(to_png, 6))?;
    module.define_singleton_method("_to_html", function!(to_html, 6))?;
    Ok(())
}