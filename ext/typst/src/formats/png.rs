use std::collections::HashMap;

use magnus::{Error, scan_args::{get_kwargs, scan_args}};

use crate::world::VirtualWorld;
use super::format::Format;

#[magnus::wrap(class = "Typst::Png")]
pub struct Png {
    compiled: PngDocument,
}

impl Png {
    pub fn new(
        text: String,
        fonts: Option<Vec<Vec<u8>>>,
        files: Option<HashMap<String, Vec<u8>>>,
        sys_inputs: Option<HashMap<String, String>>,
        render_bleed: Option<bool>,
        ppi: Option<f32>,
    ) -> Result<PngDocument, Error> {
        let world = VirtualWorld::new(text, fonts, files, sys_inputs);
        world.to_png(render_bleed, ppi)
    }
    pub fn new_ruby(
        args: &[magnus::Value],
    ) -> Result<PngDocument, Error> {
        let args = scan_args::<_, (), (), (), _, ()>(args)?;
        let (text,): (String,) = args.required;
        let kw = get_kwargs::<_, (), (
            Option<Vec<Vec<u8>>>,
            Option<HashMap<String, Vec<u8>>>,
            Option<HashMap<String, String>>,
            Option<bool>,
            Option<f32>,
        ), ()>(args.keywords, &[], &["fonts", "files", "sys_inputs", "render_bleed", "ppi"])?;
        let (fonts, files, sys_inputs, render_bleed, ppi,) = kw.optional;

        Self::new(text, fonts, files, sys_inputs, render_bleed, ppi)
    }
    pub fn compiled(&self) -> PngDocument {
        return self.compiled.clone();
    }
}

#[derive(Clone)]
#[magnus::wrap(class = "Typst::PngDocument")]
pub struct PngDocument {
    warnings: Vec<String>,
    pages: Vec<Vec<u8>>,
}

impl Format for PngDocument {}

impl PngDocument {
    pub fn new(pages: Vec<Vec<u8>>, warnings: Vec<String>) -> Self {
        let mut documents = vec![];
        for page in pages {
            documents.push(page)
        }
        PngDocument {
            warnings: warnings,
            pages: documents
        }
    }
    pub fn bytes(&self) -> Vec<Vec<u8>> {
        let mut bytes = vec![];
        for page in &self.pages {
            bytes.push(page.to_vec());
        }
        bytes
    }
    pub fn pages(&self) -> Vec<Vec<u8>> {
        self.pages.clone()
    }
    pub fn warnings(&self) -> Vec<String> {
        self.warnings.clone()
    }
    pub fn has_warnings(&self) -> bool {
        self.warnings.len() > 0
    }
    pub fn write(&self, filename: String) {
        self.write_some(self.bytes(), filename)
    }
}