use magnus::Error;
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
