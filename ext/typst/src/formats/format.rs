use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use magnus::Error;

pub trait Format {
    fn write_one(&self, bytes: Vec<u8>, filename: String) -> Result<(), Error> {
        match File::create(filename) {
            Ok(mut buffer) => Ok(buffer.write_all(&bytes).unwrap()),
            Err(error) => Err(Error::new(magnus::exception::arg_error(), error.to_string()))
        }
    }
    fn write_some(&self, pages: Vec<Vec<u8>>, filename: String) {
        let pn = PathBuf::from(&filename);
        let mut filesname = pn.file_stem().unwrap().to_string_lossy();
        if !filesname.contains("{{n}}") {
            filesname = filesname + "_{{n}}";
        }
        filesname = filesname + "." + pn.extension().unwrap().to_string_lossy();
    
        for (i, page) in pages.iter().enumerate() {
            let filen = filesname.replace("{{n}}", i.to_string().as_str());
    
            let _ = &self.write_one(page.to_vec(), filen).unwrap();
        }
    }
}
