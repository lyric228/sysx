use std::io::{Result, Write};
use std::path::Path;
use std::fs;


/// Better File
pub struct BFile {
    path: &'static str,
}

impl BFile {
    pub fn new(path: &'static str) -> Self {
        BFile { path }
    }

    /// Проверяет существование файла
    pub fn exists(&self) -> bool {
        Path::new(self.path).exists()
    }

    /// Читает содержимое файла
    pub fn read(&self) -> Result<String> {
        fs::read_to_string(self.path)
    }

    /// Добавляет данные в конец файла
    pub fn append(&self, data: &str) -> Result<()> {
        fs::OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(self.path)?
            .write_all(data.as_bytes())?;
            
        Ok(())
    }

    /// Полностью перезаписывает содержимое файла
    pub fn write(&self, data: &str) -> Result<()> {
        // Создаем родительские директории при необходимости
        if let Some(parent) = Path::new(self.path).parent() {
            fs::create_dir_all(parent)?;
        }
        
        fs::write(self.path, data)?;
        Ok(())
    }
}
