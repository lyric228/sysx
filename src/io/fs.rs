use std::{
    fs::{self, File as StdFile, OpenOptions},
    io::{self, Read, Write},
    path::{Path, PathBuf},
};

#[derive(Debug)]
pub struct File {
    path: PathBuf,
    inner: StdFile,
}

impl File {
    /// Opens a file in read mode.
    pub fn open<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let inner = StdFile::open(path.as_ref())?;
        Ok(File {
            path: path.as_ref().to_path_buf(),
            inner,
        })
    }

    /// Creates a file in write mode. If the file exists, it will be overwritten.
    pub fn create<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        if let Some(parent_dir) = path.as_ref().parent() {
            fs::create_dir_all(parent_dir)?;
        }
        let inner = StdFile::create(path.as_ref())?;
        Ok(File {
            path: path.as_ref().to_path_buf(),
            inner,
        })
    }

    /// Opens a file in append mode.
    pub fn append<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        if let Some(parent_dir) = path.as_ref().parent() {
            fs::create_dir_all(parent_dir)?;
        }
        let inner = OpenOptions::new()
            .append(true)
            .create(true)
            .open(path.as_ref())?;
        Ok(File {
            path: path.as_ref().to_path_buf(),
            inner,
        })
    }

    /// Reads the entire file contents into a String.
    pub fn read_to_string(&mut self) -> io::Result<String> {
        let mut contents = String::new();
        self.inner.read_to_string(&mut contents)?;
        Ok(contents)
    }

    /// Reads the entire file contents into a vector of bytes.
    pub fn read_to_end(&mut self) -> io::Result<Vec<u8>> {
        let mut contents = Vec::new();
        self.inner.read_to_end(&mut contents)?;
        Ok(contents)
    }

    /// Writes bytes to the file.
    pub fn write_all(&mut self, contents: &[u8]) -> io::Result<()> {
        self.inner.write_all(contents)?;
        Ok(())
    }

    /// Writes bytes to the file.
    pub fn write_bytes(&mut self, contents: &[u8]) -> io::Result<()> {
        self.write_all(contents)
    }

    /// Gets the path of the file.
    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    /// Gets a reference to the inner `std::fs::File`.
    pub fn inner(&self) -> &StdFile {
        &self.inner
    }

    /// Gets a mutable reference to the inner `std::fs::File`.
    pub fn inner_mut(&mut self) -> &mut StdFile {
        &mut self.inner
    }
}
