use std::path::PathBuf;
use std::env;

use magnus::{function, exception, Error, IntoValue};

use world::SystemWorld;

mod compiler;
mod download;
mod fonts;
mod package;
mod world;

fn compile(
    input: PathBuf,
    output: Option<PathBuf>,
    root: Option<PathBuf>,
    font_paths: Vec<PathBuf>,
) -> Result<magnus::Value, Error> {
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

    let resource_path = env::current_dir()
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

    if let Some(output) = output {
        std::fs::write(output, pdf_bytes)
            .map_err(|_| magnus::Error::new(exception::arg_error(), "error"))?;

        let value = true.into_value();
        Ok(value)
    } else {
        let value = pdf_bytes.into_value(); 
        Ok(value)
    }
}

#[magnus::init]
fn init() {
    let module = magnus::define_module("Typst").unwrap();
    module.define_module_function("compile", function!(compile, 4)).unwrap();
}