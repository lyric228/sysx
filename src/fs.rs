use std::path::{Component, Path, PathBuf};
use std::env::current_dir as cur_dir;
use std::fs::{self, OpenOptions};
use std::io::{Result, Write};

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

#[cfg(windows)]
use winapi::um::winnt::{
    FILE_ATTRIBUTE_ARCHIVE,
    FILE_ATTRIBUTE_HIDDEN,
    FILE_ATTRIBUTE_READONLY,
    FILE_ATTRIBUTE_SYSTEM,
};
#[cfg(windows)]
use winapi::um::fileapi::{GetFileAttributesW, SetFileAttributesW};

/// Structure for file operations.
///
/// Stores the path to the file as a `PathBuf`.
pub struct BFile {
    path: PathBuf,
}

impl BFile {
    /// Creates a new instance of `BFile`.
    ///
    /// If the provided path is relative, it is converted into an absolute path
    /// using the current working directory.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the file to be processed.
    ///
    /// # Example
    /// ```rust
    /// let file = BFile::new("test.txt").unwrap();
    /// ```
    pub fn new<P: Into<PathBuf>>(path: P) -> Result<Self> {
        let mut pathbuf: PathBuf = path.into();
        if pathbuf.is_relative() {
            pathbuf = cur_dir()?.join(pathbuf);
        }
        let normalized = normalize_path(&pathbuf);
        Ok(BFile { path: normalized })
    }

    /// Checks whether the file exists.
    ///
    /// # Example
    /// ```rust
    /// let file = BFile::new("test.txt").unwrap();
    /// if file.exists() {
    ///     println!("File exists");
    /// }
    /// ```
    pub fn exists(&self) -> bool {
        self.path.exists()
    }

    /// Reads the file content as a string.
    ///
    /// # Example
    /// ```rust
    /// let file = BFile::new("test.txt").unwrap();
    /// let content = file.read().unwrap();
    /// println!("{}", content);
    /// ```
    pub fn read(&self) -> Result<String> {
        fs::read_to_string(&self.path)
    }

    /// Appends data to the end of the file.
    ///
    /// If the file does not exist, it will be created.
    ///
    /// # Arguments
    ///
    /// * `data` - The string data to append.
    ///
    /// # Example
    /// ```rust
    /// let file = BFile::new("test.txt").unwrap();
    /// file.append("New data\n").unwrap();
    /// ```
    pub fn append(&self, data: &str) -> Result<()> {
        OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(&self.path)?
            .write_all(data.as_bytes())?;
        Ok(())
    }

    /// Writes data to the file, replacing its content.
    ///
    /// If the necessary directories do not exist, they will be created.
    ///
    /// # Arguments
    ///
    /// * `data` - The string data to write.
    ///
    /// # Example
    /// ```rust
    /// let file = BFile::new("test.txt").unwrap();
    /// file.write("New content").unwrap();
    /// ```
    pub fn write(&self, data: &str) -> Result<()> {
        if let Some(parent) = self.path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(&self.path, data)?;
        Ok(())
    }

    /// Deletes the file if it exists.
    ///
    /// # Example
    /// ```rust
    /// let file = BFile::new("test.txt").unwrap();
    /// file.delete().unwrap();
    /// ```
    pub fn delete(&self) -> Result<()> {
        if self.exists() {
            fs::remove_file(&self.path)?;
        }
        Ok(())
    }

    /// Renames the file.
    ///
    /// If the provided new path is relative, it is interpreted relative to the file's current directory.
    /// If the parent directory of the new path does not exist, it will be created.
    ///
    /// # Arguments
    ///
    /// * `new_path` - The new path (relative or absolute) for the file.
    ///
    /// # Example
    /// ```rust
    /// let mut file = BFile::new("old_name.txt").unwrap();
    /// file.rename("new_name.txt").unwrap();
    /// ```
    pub fn rename<P: Into<PathBuf>>(&mut self, new_path: P) -> Result<()> {
        let new_path_raw: PathBuf = new_path.into();
        let new_full_path = if new_path_raw.is_relative() {
            if let Some(parent) = self.path.parent() {
                parent.join(new_path_raw)
            } else {
                cur_dir()?.join(new_path_raw)
            }
        } else {
            new_path_raw
        };

        let new_full_path = normalize_path(&new_full_path);

        if let Some(parent) = new_full_path.parent() {
            fs::create_dir_all(parent)?;
        }

        fs::rename(&self.path, &new_full_path)?;
        self.path = new_full_path;
        Ok(())
    }

    /// Returns a reference to the file's path.
    ///
    /// # Example
    /// ```rust
    /// let file = BFile::new("test.txt").unwrap();
    /// println!("{:?}", file.path());
    /// ```
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Retrieves the file's permissions and returns them as a string.
    ///
    /// On Unix systems, an octal representation is returned (e.g., "755").
    /// On Windows systems, returns file attributes:
    ///   - 'R' if the file is read-only,
    ///   - 'H' if the file is hidden,
    ///   - 'S' if the file is a system file,
    ///   - if none of the attributes apply, returns 'A' (archive).
    ///
    /// # Example
    /// ```rust
    /// let file = BFile::new("test.txt").unwrap();
    /// println!("Permissions: {}", file.get_permissions().unwrap());
    /// ```
    pub fn get_permissions(&self) -> Result<String> {
        #[cfg(unix)]
        {
            let meta = fs::metadata(&self.path)?;
            Ok(format!("{:o}", meta.permissions().mode() & 0o777))
        }
        #[cfg(windows)]
        {
            use std::ffi::OsStr;
            use std::os::windows::ffi::OsStrExt;
            
            // Convert path to wide string with null terminator
            let path_w: Vec<u16> = self.path.as_os_str().encode_wide().chain(Some(0)).collect();
            let attrs = unsafe { GetFileAttributesW(path_w.as_ptr()) };
            if attrs == u32::MAX {
                return Err(std::io::Error::last_os_error());
            }
            let mut s = String::new();
            if attrs & FILE_ATTRIBUTE_READONLY != 0 { s.push('R'); }
            if attrs & FILE_ATTRIBUTE_HIDDEN != 0 { s.push('H'); }
            if attrs & FILE_ATTRIBUTE_SYSTEM != 0 { s.push('S'); }
            if s.is_empty() { s.push('A'); } // Default: Archive
            Ok(s)
        }
    }

    /// Sets the file's permissions based on the provided string.
    ///
    /// For Unix systems, the string should represent an octal number (e.g., "755").
    /// For Windows systems, the following characters are used:
    ///   - 'R' or 'r' for read-only,
    ///   - 'H' or 'h' for hidden,
    ///   - 'S' or 's' for a system file.
    ///
    /// # Arguments
    ///
    /// * `perm_str` - The string defining the file permissions.
    ///
    /// # Example
    /// ```rust
    /// let file = BFile::new("test.txt").unwrap();
    /// file.set_permissions("755").unwrap(); // Unix
    /// file.set_permissions("RHS").unwrap(); // Windows
    /// ```
    pub fn set_permissions(&self, perm_str: &str) -> Result<()> {
        #[cfg(unix)]
        {
            let mode = u32::from_str_radix(perm_str, 8).map_err(|_| {
                std::io::Error::new(std::io::ErrorKind::InvalidInput, "Incorrect Unix rights format.")
            })?;
            let permissions = fs::Permissions::from_mode(mode);
            fs::set_permissions(&self.path, permissions)
        }
        #[cfg(windows)]
        {
            let mut new_attrs = FILE_ATTRIBUTE_ARCHIVE;
            for ch in perm_str.chars() {
                match ch {
                    'R' | 'r' => new_attrs |= FILE_ATTRIBUTE_READONLY,
                    'H' | 'h' => new_attrs |= FILE_ATTRIBUTE_HIDDEN,
                    'S' | 's' => new_attrs |= FILE_ATTRIBUTE_SYSTEM,
                    _ => {
                        return Err(std::io::Error::new(
                            std::io::ErrorKind::InvalidInput,
                            "Invalid attribute character for Windows.",
                        ));
                    }
                }
            }
            use std::os::windows::ffi::OsStrExt;
            let path_w: Vec<u16> = self.path.as_os_str().encode_wide().chain(Some(0)).collect();
            let res = unsafe { SetFileAttributesW(path_w.as_ptr(), new_attrs) };
            if res == 0 {
                return Err(std::io::Error::last_os_error());
            }
            Ok(())
        }
    }
    
    /// Retrieves the file's metadata.
    ///
    /// # Example
    /// ```rust
    /// let file = BFile::new("test.txt").unwrap();
    /// let metadata = file.get_metadata().unwrap();
    /// println!("Size: {}", metadata.len());
    /// ```
    pub fn get_metadata(&self) -> Result<fs::Metadata> {
        fs::metadata(&self.path)
    }
}

/// Normalizes a path by removing redundant components,
/// such as "." and "..".
///
/// # Arguments
///
/// * `path` - A reference to the path to be normalized.
///
/// # Example
/// ```rust
/// let normalized = normalize_path("/home/user/../user/docs/./file.txt");
/// println!("{:?}", normalized);
/// ```
pub fn normalize_path<P: AsRef<Path>>(path: P) -> PathBuf {
    let mut normalized = PathBuf::new();
    for component in path.as_ref().components() {
        match component {
            Component::CurDir => continue,
            Component::ParentDir => {
                normalized.pop();
            }
            _ => normalized.push(component.as_os_str()),
        }
    }
    normalized
}


pub fn get_dir_size(path: &str) -> std::io::Result<u64> {
    let mut total = 0;

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let meta = entry.metadata()?;
        if meta.is_dir() {
            total += get_dir_size(entry.path().to_str().unwrap())?;
        } else {
            total += meta.len();
        }
    }
    
    Ok(total)
}
