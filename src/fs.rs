use std::io::{Result, Write};
use std::path::Path;
use std::fs;


pub struct BFile {
    path: &'static str,
}

impl BFile {
    pub fn new(path: &'static str) -> Self {
        BFile { path }
    }

    pub fn exists(&self) -> bool {
        Path::new(self.path).exists()
    }

    pub fn read(&self) -> Result<String> {
        fs::read_to_string(self.path)
    }

    pub fn append(&self, data: &str) -> Result<()> {
        fs::OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(self.path)?
            .write_all(data.as_bytes())?;
            
        Ok(())
    }

    pub fn write(&self, data: &str) -> Result<()> {
        if let Some(parent) = Path::new(self.path).parent() {
            fs::create_dir_all(parent)?;
        }
        
        fs::write(self.path, data)?;
        Ok(())
    }
}
