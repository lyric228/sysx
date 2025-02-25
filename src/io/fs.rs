use std::env::current_dir as cur_dir;
use std::fs::{self, OpenOptions};
use std::io::{Result, Write};
use std::path::{Component, Path, PathBuf};

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

/// Структура для работы с файлами в Unix-подобных системах
///
/// # Example
/// ```no_run
/// use sysx::fs::BFile;
///
/// let file = BFile::new("example.txt")?;
/// file.write("Hello, World!")?;
/// ```
pub struct BFile {
    path: PathBuf,
}

impl BFile {
    /// Создает новый экземпляр BFile с указанным путем
    ///
    /// # Arguments
    /// * `path` - Путь к файлу (относительный или абсолютный)
    ///
    /// # Returns
    /// * `Result<BFile>` - Результат создания файла
    ///
    /// # Example
    /// ```no_run
    /// use sysx::fs::BFile;
    ///
    /// let file = BFile::new("docs/example.txt")?;
    /// ```
    pub fn new<P: Into<PathBuf>>(path: P) -> Result<Self> {
        let mut pathbuf: PathBuf = path.into();
        if pathbuf.is_relative() {
            pathbuf = cur_dir()?.join(pathbuf);
        }
        let normalized = normalize_path(&pathbuf);
        Ok(BFile { path: normalized })
    }

    /// Проверяет существование файла
    ///
    /// # Returns
    /// * `bool` - true если файл существует, false если нет
    ///
    /// # Example
    /// ```no_run
    /// use sysx::fs::BFile;
    ///
    /// let file = BFile::new("example.txt")?;
    /// if file.exists() {
    ///     println!("Файл существует");
    /// }
    /// ```
    pub fn exists(&self) -> bool {
        self.path.exists()
    }

    /// Читает содержимое файла в строку UTF-8
    ///
    /// # Returns
    /// * `Result<String>` - Содержимое файла или ошибка чтения
    ///
    /// # Example
    /// ```no_run
    /// use sysx::fs::BFile;
    ///
    /// let file = BFile::new("example.txt")?;
    /// let content = file.read()?;
    /// println!("{}", content);
    /// ```
    pub fn read(&self) -> Result<String> {
        fs::read_to_string(&self.path)
    }

    /// Добавляет данные в конец файла
    ///
    /// # Arguments
    /// * `data` - Строка для добавления в файл
    ///
    /// # Returns
    /// * `Result<()>` - Результат операции добавления
    ///
    /// # Example
    /// ```no_run
    /// use sysx::fs::BFile;
    ///
    /// let file = BFile::new("log.txt")?;
    /// file.append("Новая строка лога\n")?;
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

    /// Записывает данные в файл, заменяя текущее содержимое
    ///
    /// # Arguments
    /// * `data` - Строка для записи в файл
    ///
    /// # Returns
    /// * `Result<()>` - Результат операции записи
    ///
    /// # Example
    /// ```no_run
    /// use sysx::fs::BFile;
    ///
    /// let file = BFile::new("config.txt")?;
    /// file.write("новая конфигурация")?;
    /// ```
    pub fn write(&self, data: &str) -> Result<()> {
        if let Some(parent) = self.path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(&self.path, data)?;
        Ok(())
    }

    /// Удаляет файл, если он существует
    ///
    /// # Returns
    /// * `Result<()>` - Результат операции удаления
    ///
    /// # Example
    /// ```no_run
    /// use sysx::fs::BFile;
    ///
    /// let file = BFile::new("temp.txt")?;
    /// file.delete()?;
    /// ```
    pub fn delete(&self) -> Result<()> {
        if self.exists() {
            fs::remove_file(&self.path)?;
        }
        Ok(())
    }

    /// Переименовывает файл и обновляет внутренний путь
    ///
    /// # Arguments
    /// * `new_path` - Новый путь для файла
    ///
    /// # Returns
    /// * `Result<()>` - Результат операции переименования
    ///
    /// # Example
    /// ```no_run
    /// use sysx::fs::BFile;
    ///
    /// let mut file = BFile::new("old.txt")?;
    /// file.rename("new.txt")?;
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
    /// Возвращает ссылку на текущий путь файла
    ///
    /// # Returns
    /// * `&Path` - Ссылка на путь файла
    ///
    /// # Example
    /// ```no_run
    /// use sysx::fs::BFile;
    ///
    /// let file = BFile::new("data.txt")?;
    /// println!("Путь к файлу: {:?}", file.path());
    /// ```
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Получает права доступа файла в восьмеричном формате (только Unix)
    ///
    /// # Returns
    /// * `Result<String>` - Строка с правами доступа в формате "644"
    ///
    /// # Example
    /// ```no_run
    /// use sysx::fs::BFile;
    ///
    /// let file = BFile::new("script.sh")?;
    /// println!("Права доступа: {}", file.get_permissions()?);
    /// ```
    #[cfg(unix)]
    pub fn get_permissions(&self) -> Result<String> {
        let meta = fs::metadata(&self.path)?;
        Ok(format!("{:o}", meta.permissions().mode() & 0o777))
    }

    /// Устанавливает права доступа файла (только Unix)
    ///
    /// # Arguments
    /// * `perm_str` - Строка с правами в восьмеричном формате (например, "755")
    ///
    /// # Returns
    /// * `Result<()>` - Результат установки прав
    ///
    /// # Example
    /// ```no_run
    /// use sysx::fs::BFile;
    ///
    /// let file = BFile::new("script.sh")?;
    /// file.set_permissions("755")?; // Установить права на исполнение
    /// ```
    #[cfg(unix)]
    pub fn set_permissions(&self, perm_str: &str) -> Result<()> {
        let mode = u32::from_str_radix(perm_str, 8).map_err(|_| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Некорректный формат прав доступа Unix",
            )
        })?;
        let permissions = fs::Permissions::from_mode(mode);
        fs::set_permissions(&self.path, permissions)
    }

    /// Получает метаданные файла
    ///
    /// # Returns
    /// * `Result<fs::Metadata>` - Метаданные файла или ошибка
    ///
    /// # Example
    /// ```no_run
    /// use sysx::fs::BFile;
    ///
    /// let file = BFile::new("data.txt")?;
    /// let metadata = file.get_metadata()?;
    /// println!("Размер файла: {} байт", metadata.len());
    /// ```
    pub fn get_metadata(&self) -> Result<fs::Metadata> {
        fs::metadata(&self.path)
    }
}

/// Нормализует путь, удаляя избыточные компоненты вроде "." и ".."
///
/// # Arguments
/// * `path` - Исходный путь для нормализации
///
/// # Returns
/// * `PathBuf` - Нормализованный путь
///
/// # Example
/// ```no_run
/// use sysx::fs::normalize_path;
/// use std::path::PathBuf;
///
/// let path = normalize_path("./docs/../docs/file.txt");
/// assert_eq!(path, PathBuf::from("docs/file.txt"));
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

/// Рекурсивно вычисляет размер директории
///
/// # Arguments
/// * `path` - Путь к директории
///
/// # Returns
/// * `Result<u64>` - Размер директории в байтах
///
/// # Example
/// ```no_run
/// use sysx::fs::get_dir_size;
///
/// let size = get_dir_size("/home/user/docs")?;
/// println!("Размер директории: {} байт", size);
/// ```
pub fn get_dir_size(path: &str) -> Result<u64> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_file_operations() -> Result<()> {
        let mut temp_file = NamedTempFile::new()?;
        let test_content = "тестовое содержимое";
        write!(temp_file, "{}", test_content)?;

        let bfile = BFile::new(temp_file.path())?;
        assert_eq!(bfile.read()?, test_content);
        assert!(bfile.exists());

        #[cfg(unix)]
        {
            bfile.set_permissions("644")?;
            assert_eq!(bfile.get_permissions()?, "644");
        }

        Ok(())
    }
}
