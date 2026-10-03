use magnus::{ RString, RArray, Error, Ruby };
use super::format::Format;

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
    pub fn document(&self) -> Result<RString, Error> {
        match Ruby::get() {
            Ok(ruby) => {
                Ok(ruby.str_from_slice(self.bytes.as_slice()))
            },
            Err(_) => panic!("Ruby Unavailable")
        }
    }
    pub fn bytes(&self) -> Result<RArray, Error> {
        match Ruby::get() {
            Ok(ruby) => {
                Ok(ruby.ary_from_iter(self.bytes.clone()))
            },
            Err(_) => panic!("Ruby Unavailable")
        }
    }
    pub fn warnings(&self) -> Vec<String> {
        self.warnings.clone()
    }
    pub fn has_warnings(&self) -> bool {
        self.warnings.len() > 0
    }
    pub fn write(&self, filename: String) -> Result<(), Error> {
        self.write_one(self.bytes.clone(), filename)
    }
}
