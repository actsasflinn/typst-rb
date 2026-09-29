use super::format::Format;

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
