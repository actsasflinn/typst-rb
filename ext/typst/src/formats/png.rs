use super::format::Format;

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