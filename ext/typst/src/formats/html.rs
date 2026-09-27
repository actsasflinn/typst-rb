use std::collections::HashMap;

use magnus::{Error, scan_args::{get_kwargs, scan_args}};

use crate::world::VirtualWorld;
use super::format::Format;

#[magnus::wrap(class = "Typst::Html")]
pub struct Html {
    compiled: HtmlExperimentalDocument,
}

impl Html {
    pub fn new(
        text: String,
        fonts: Option<Vec<Vec<u8>>>,
        files: Option<HashMap<String, Vec<u8>>>,
        sys_inputs: Option<HashMap<String, String>>,
        pretty: Option<bool>,
    ) -> Result<HtmlExperimentalDocument, Error> {
        let world = VirtualWorld::new(text, fonts, files, sys_inputs);
        world.to_html(pretty)
    }
    pub fn new_ruby(
        args: &[magnus::Value],
    ) -> Result<HtmlExperimentalDocument, Error> {
        let args = scan_args::<_, (), (), (), _, ()>(args)?;
        let (text,): (String,) = args.required;
        let kw = get_kwargs::<_, (), (
            Option<Vec<Vec<u8>>>,
            Option<HashMap<String, Vec<u8>>>,
            Option<HashMap<String, String>>,
            Option<bool>,
        ), ()>(args.keywords, &[], &["fonts", "files", "sys_inputs", "pretty"])?;
        let (fonts, files, sys_inputs, pretty,) = kw.optional;

        Self::new(text, fonts, files, sys_inputs, pretty)
    }
    pub fn compiled(&self) -> HtmlExperimentalDocument {
        return self.compiled.clone();
    }
}

#[derive(Clone)]
#[magnus::wrap(class = "Typst::HtmlExperimentalDocument")]
pub struct HtmlExperimentalDocument {
    document: String,
    warnings: Vec<String>
}

impl Format for HtmlExperimentalDocument {}

impl HtmlExperimentalDocument {
    pub fn new(document: String, warnings: Vec<String>) -> Self {
        HtmlExperimentalDocument {
            document: document,
            warnings: warnings
        }  
    }
    pub fn document(&self) -> String {
        self.document.clone()
    }
    pub fn bytes(&self) -> Vec<u8> {
        self.document.as_bytes().to_vec()
    }
    pub fn warnings(&self) -> Vec<String> {
        self.warnings.clone()
    }
    pub fn has_warnings(&self) -> bool {
        self.warnings.len() > 0
    }
    pub fn write(&self, filename: String) -> Result<(), Error> {
        self.write_one(self.bytes(), filename)
    }
}
