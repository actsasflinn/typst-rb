use std::collections::HashMap;

use magnus::{Error, scan_args::{get_kwargs, scan_args}};

use crate::world::VirtualWorld;
use super::format::Format;

#[magnus::wrap(class = "Typst::Svg")]
pub struct Svg {
    compiled: SvgDocument,
}

impl Svg {
    pub fn new(
        text: String,
        fonts: Option<Vec<Vec<u8>>>,
        files: Option<HashMap<String, Vec<u8>>>,
        sys_inputs: Option<HashMap<String, String>>,
        pretty: Option<bool>,
        render_bleed: Option<bool>,
    ) -> Result<SvgDocument, Error> {
        let world = VirtualWorld::new(text, fonts, files, sys_inputs);
        world.to_svg(pretty, render_bleed)
    }
    pub fn new_ruby(
        args: &[magnus::Value],
    ) -> Result<SvgDocument, Error> {
        let args = scan_args::<_, (), (), (), _, ()>(args)?;
        let (text,): (String,) = args.required;
        let kw = get_kwargs::<_, (), (
            Option<Vec<Vec<u8>>>,
            Option<HashMap<String, Vec<u8>>>,
            Option<HashMap<String, String>>,
            Option<bool>,
            Option<bool>,
        ), ()>(args.keywords, &[], &["fonts", "files", "sys_inputs", "pretty", "render_bleed"])?;
        let (fonts, files, sys_inputs, pretty, render_bleed,) = kw.optional;

        Self::new(text, fonts, files, sys_inputs, pretty, render_bleed)
    }
    pub fn compiled(&self) -> SvgDocument {
        return self.compiled.clone();
    }
}

#[derive(Clone)]
#[magnus::wrap(class = "Typst::SvgDocument")]
pub struct SvgDocument {
    warnings: Vec<String>,
    pages: Vec<String>,
}

impl Format for SvgDocument {}

impl SvgDocument {
    pub fn new(pages: Vec<String>, warnings: Vec<String>) -> Self {
        let mut documents = vec![];
        for page in pages {
            documents.push(page)
        }
        SvgDocument {
            warnings: warnings,
            pages: documents
        }
    }
    pub fn bytes(&self) -> Vec<Vec<u8>> {
        let mut bytes = vec![];
        for page in &self.pages {
            bytes.push(page.as_bytes().to_vec());
        }
        bytes
    }
    pub fn pages(&self) -> Vec<String> {
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
