use magnus::Error;
use super::format::Format;

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
