use std::collections::HashMap;

use magnus::{Error, scan_args::{get_kwargs, scan_args}};

use crate::world::VirtualWorld;
use super::format::Format;

#[magnus::wrap(class = "Typst::Pdf")]
pub struct Pdf {
    compiled: PdfDocument,
}

impl Pdf {
    pub fn new(
        text: String,
        fonts: Option<Vec<Vec<u8>>>,
        files: Option<HashMap<String, Vec<u8>>>,
        sys_inputs: Option<HashMap<String, String>>,
        pretty: Option<bool>,
        pdf_standards: Option<Vec<String>>,
    ) -> Result<PdfDocument, Error> {
        let world = VirtualWorld::new(text, fonts, files, sys_inputs);
        world.to_pdf(pdf_standards, pretty)
    }
    pub fn new_ruby(
        args: &[magnus::Value],
    ) -> Result<PdfDocument, Error> {
        let args = scan_args::<_, (), (), (), _, ()>(args)?;
        let (text,): (String,) = args.required;
        let kw = get_kwargs::<_, (), (
            Option<Vec<Vec<u8>>>,
            Option<HashMap<String, Vec<u8>>>,
            Option<HashMap<String, String>>,
            Option<bool>,
            Option<Vec<String>>,
        ), ()>(args.keywords, &[], &["fonts", "files", "sys_inputs", "pretty", "pdf_standards"])?;
        let (fonts, files, sys_inputs, pretty, pdf_standards,) = kw.optional;

        Self::new(text, fonts, files, sys_inputs, pretty, pdf_standards)
    }
    pub fn compiled(&self) -> PdfDocument {
        return self.compiled.clone();
    }
}

#[derive(Clone)]
#[magnus::wrap(class = "Typst::PdfDocument")]
pub struct PdfDocument {
    bytes: Vec<u8>,
    warnings: Vec<String>
}

impl Format for PdfDocument {}

impl PdfDocument {
    pub fn new(bytes: Vec<u8>, warnings: Vec<String>) -> Self {
        PdfDocument {
            bytes: bytes,
            warnings: warnings
        }  
    }
    pub fn bytes(&self) -> Vec<u8> {
        self.bytes.clone()
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
