use std::fs as std_fs;
use std::path::Path;


pub struct File {
    path: &'static str
}

impl File {
    pub fn new(path: &'static str) -> Self {
        let file = File {
            path
        };
        if !file.exists() {
            let _ = std_fs::File::create(path);
        }
        file
    }

    pub fn exists(&self) -> bool {
        Path::new(&self.path).exists()
    }

    pub fn read(&self) -> String {
        std_fs::read_to_string(&self.path).unwrap()
    }

    pub fn write(&self, data: &str) {
        let _ = &self.overwrite(format!("{}{}", &self.read(), data).as_str());
    }

    pub fn overwrite(&self, data: &str) {
        let _ = std_fs::write(&self.path, data);
    }
}
