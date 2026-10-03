use magnus::{ RArray, Error, Ruby };
use super::format::Format;

#[derive(Clone)]
#[magnus::wrap(class = "Typst::PngDocument")]
pub struct PngDocument {
    warnings: Vec<String>,
    //pages: Vec<Vec<u8>>,
    bytes: Vec<Vec<u8>>,
}

impl Format for PngDocument {}

impl PngDocument {
    pub fn new(pages: Vec<Vec<u8>>, warnings: Vec<String>) -> Self {
        let mut bytes = vec![];
        for page in pages {
            bytes.push(page)
        }
        PngDocument {
            warnings: warnings,
            bytes: bytes
        }
    }
    pub fn bytes(&self) -> Result<RArray, Error> {
        let mut bytes = vec![];
        match Ruby::get() {
            Ok(ruby) => {
                for page in &self.bytes {
                    bytes.push(ruby.ary_from_iter(page.to_vec()))
                }
                Ok(ruby.ary_from_iter(bytes))
            },
            Err(_) => panic!("Ruby Unavailable")    
        }
    }
    pub fn pages(&self) -> Result<RArray, Error> {
        let mut bytes = vec![];
        match Ruby::get() {
            Ok(ruby) => {
                for page in &self.bytes {
                    bytes.push(ruby.str_from_slice(page.as_slice()))
                }
                Ok(ruby.ary_from_iter(bytes))
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
    pub fn write(&self, filename: String) {
        self.write_some(self.bytes.clone(), filename)
    }
}