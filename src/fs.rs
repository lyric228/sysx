use std::path::{Component, Path, PathBuf};
use std::fs::{self, OpenOptions};
use std::io::{Result, Write};
use std::env;

pub use std::io::Error;

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

#[cfg(windows)]
use winapi::um::winnt::{FILE_ATTRIBUTE_ARCHIVE, FILE_ATTRIBUTE_HIDDEN, FILE_ATTRIBUTE_READONLY, FILE_ATTRIBUTE_SYSTEM};
#[cfg(windows)]
use winapi::um::fileapi::GetFileAttributesW;
#[cfg(windows)]
use winapi::um::fileapi::SetFileAttributesW;

/// Структура для работы с файлами.
/// 
/// Хранит путь к файлу в виде `PathBuf`.
pub struct BFile {
    path: PathBuf,
}

impl BFile {
    /// Создает новый экземпляр структуры `BFile`.
    ///
    /// Если переданный путь относительный, он преобразуется в абсолютный, 
    /// используя текущую рабочую директорию.
    ///
    /// # Аргументы
    /// * `path` - Путь к файлу, который требуется обработать.
    ///
    /// # Пример
    /// ```rust
    /// let file = BFile::new("test.txt").unwrap();
    /// ```
    pub fn new<P: Into<PathBuf>>(path: P) -> Result<Self> {
        let mut pathbuf: PathBuf = path.into();
        if pathbuf.is_relative() {
            pathbuf = env::current_dir()?.join(pathbuf);
        }
        let normalized = normalize_path(&pathbuf);
        Ok(BFile { path: normalized })
    }

    /// Проверяет, существует ли файл.
    ///
    /// # Пример
    /// ```rust
    /// let file = BFile::new("test.txt").unwrap();
    /// if file.exists() {
    ///     println!("Файл существует");
    /// }
    /// ```
    pub fn exists(&self) -> bool {
        self.path.exists()
    }

    /// Читает содержимое файла в виде строки.
    ///
    /// # Пример
    /// ```rust
    /// let file = BFile::new("test.txt").unwrap();
    /// let content = file.read().unwrap();
    /// println!("{}", content);
    /// ```
    pub fn read(&self) -> Result<String> {
        fs::read_to_string(&self.path)
    }

    /// Добавляет данные в конец файла.
    /// Если файл не существует, он будет создан.
    ///
    /// # Аргументы
    /// * `data` - Строка с данными для добавления.
    ///
    /// # Пример
    /// ```rust
    /// let file = BFile::new("test.txt").unwrap();
    /// file.append("Новые данные\n").unwrap();
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

    /// Записывает данные в файл, перезаписывая его содержимое.
    /// Если необходимые директории не существуют, они будут созданы.
    ///
    /// # Аргументы
    /// * `data` - Строка с данными для записи.
    ///
    /// # Пример
    /// ```rust
    /// let file = BFile::new("test.txt").unwrap();
    /// file.write("Новое содержимое").unwrap();
    /// ```
    pub fn write(&self, data: &str) -> Result<()> {
        if let Some(parent) = self.path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(&self.path, data)?;
        Ok(())
    }

    /// Удаляет файл, если он существует.
    ///
    /// # Пример
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

    /// Переименовывает файл.
    ///
    /// Если переданный новый путь относительный, он будет интерпретирован относительно текущей директории файла.
    /// Если родительский каталог нового пути не существует, он будет создан.
    ///
    /// # Аргументы
    /// * `new_path` - Новый путь (относительный или абсолютный) для файла.
    ///
    /// # Пример
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
                env::current_dir()?.join(new_path_raw)
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

    /// Возвращает ссылку на путь файла.
    ///
    /// # Пример
    /// ```rust
    /// let file = BFile::new("test.txt").unwrap();
    /// println!("{:?}", file.path());
    /// ```
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Получает права доступа к файлу и возвращает их в виде строки.
    ///
    /// На Unix системах возвращается восьмеричное представление (например, "755").
    /// На Windows системах возвращаются атрибуты файла:
    ///   - 'R' если файл доступен только для чтения,
    ///   - 'H' если файл скрытый,
    ///   - 'S' если файл системный,
    ///   - если атрибуты не определены, возвращается 'A' (архивный).
    ///
    /// # Пример
    /// ```rust
    /// let file = BFile::new("test.txt").unwrap();
    /// println!("Права: {}", file.get_permissions().unwrap());
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

            let path_w: Vec<u16> = self.path.as_os_str().encode_wide().chain(Some(0)).collect();
            let attrs = unsafe { GetFileAttributesW(path_w.as_ptr()) };

            if attrs == u32::MAX {
                return Err(std::io::Error::last_os_error());
            }

            let mut s = String::new();

            if attrs & FILE_ATTRIBUTE_READONLY != 0 { s.push('R'); }
            if attrs & FILE_ATTRIBUTE_HIDDEN != 0 { s.push('H'); }
            if attrs & FILE_ATTRIBUTE_SYSTEM != 0 { s.push('S'); }
            if s.is_empty() { s.push('A'); } // Архивный по умолчанию

            Ok(s)
        }
    }

    /// Устанавливает права доступа к файлу на основе переданной строки.
    ///
    /// Для Unix систем строка должна представлять восьмеричное число (например, "755").
    /// Для Windows систем используются символы:
    ///   - 'R' или 'r' для чтения,
    ///   - 'H' или 'h' для скрытого файла,
    ///   - 'S' или 's' для системного файла.
    ///
    /// # Аргументы
    /// * `perm_str` - Строка, задающая права доступа.
    ///
    /// # Пример
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

            let path_w: Vec<u16> = self.path.as_os_str().encode_wide().chain(Some(0)).collect();
            let res = unsafe { SetFileAttributesW(path_w.as_ptr(), new_attrs) };

            if res == 0 {
                return Err(std::io::Error::last_os_error());
            }

            Ok(())
        }
    }
    
    /// Получает метаданные файла.
    ///
    /// # Пример
    /// ```rust
    /// let file = BFile::new("test.txt").unwrap();
    /// let metadata = file.get_metadata().unwrap();
    /// println!("Размер: {}", metadata.len());
    /// ```
    pub fn get_metadata(&self) -> Result<fs::Metadata> {
        fs::metadata(&self.path)
    }
}


/// Нормализует путь, убирая из него избыточные компоненты,
/// такие как "." и "..".
///
/// # Аргументы
/// * `path` - Ссылка на путь для нормализации.
///
/// # Пример
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
