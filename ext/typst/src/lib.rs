use std::path::PathBuf;

use magnus::{define_class, class, function, exception, Error, IntoValue};
use magnus::{prelude::*};

use world::SystemWorld;

mod compiler;
mod download;
mod fonts;
mod package;
mod world;

fn compile(
    input: PathBuf,
    root: Option<PathBuf>,
    font_paths: Vec<PathBuf>,
    resource_path: PathBuf,
) -> Result<Vec<u8>, Error> {
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
        .font_paths(font_paths)
        .font_files(default_fonts)
        .build()
        .map_err(|msg| magnus::Error::new(exception::arg_error(), msg.to_string()))?;

    let pdf_bytes = world
        .compile()
        .map_err(|msg| magnus::Error::new(exception::arg_error(), msg.to_string()))?;

    Ok(pdf_bytes)
}

fn write(
    input: PathBuf,
    output: PathBuf,
    root: Option<PathBuf>,
    font_paths: Vec<PathBuf>,
    resource_path: PathBuf,
) -> Result<magnus::Value, Error> {
    let pdf_bytes = compile(input, root, font_paths, resource_path)?;

    std::fs::write(output, pdf_bytes)
        .map_err(|_| magnus::Error::new(exception::arg_error(), "error"))?;

    let value = true.into_value();
    Ok(value)
}    

#[magnus::init]
fn init() {
    env_logger::init();

    let class = define_class("Typst", class::object()).unwrap();
    class.define_singleton_method("_compile", function!(compile, 4)).unwrap();
    class.define_singleton_method("_write", function!(write, 5)).unwrap();
}