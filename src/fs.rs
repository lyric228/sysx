use std::path::{Component, Path, PathBuf};
use std::fs::{self, OpenOptions};
use std::io::{Result, Write};
use std::env;


/// Структура BFile представляет собой абстракцию для работы с файлами.
/// Она предоставляет методы для записи, чтения, дописывания, удаления и переименования файлов.
pub struct BFile {
    path: PathBuf,
}

impl BFile {
    /// Создаёт новый объект BFile для файла по указанному пути.
    /// Если путь относительный, он превращается в абсолютный относительно текущей рабочей директории.
    ///
    /// # Пример
    /// 
    /// ```rust
    /// let file = BFile::new("/home/user/Downloads/abc/file.txt").expect("Ошибка создания BFile");
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
    /// 
    /// ```rust
    /// if file.exists() {
    ///     println!("Файл существует!");
    /// }
    /// ```
    pub fn exists(&self) -> bool {
        self.path.exists()
    }

    /// Читает содержимое файла и возвращает его в виде строки.
    ///
    /// # Пример
    /// 
    /// ```rust
    /// let content = file.read().expect("Ошибка чтения файла");
    /// println!("{}", content);
    /// ```
    pub fn read(&self) -> Result<String> {
        fs::read_to_string(&self.path)
    }

    /// Дописывает данные в конец файла.
    /// Если файл не существует, он будет создан.
    ///
    /// # Пример
    /// 
    /// ```rust
    /// file.append("\nНовые данные").expect("Ошибка дописывания данных");
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
    /// Если родительские каталоги не существуют, они будут созданы.
    ///
    /// # Пример
    /// 
    /// ```rust
    /// file.write("Новые данные").expect("Ошибка записи в файл");
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
    ///
    /// ```rust
    /// file.delete().expect("Ошибка удаления файла");
    /// ```
    pub fn delete(&self) -> Result<()> {
        if self.exists() {
            fs::remove_file(&self.path)?;
        }
        Ok(())
    }

    /// Переименовывает файл, обновляя внутреннее поле пути объекта BFile.
    /// Если переданный путь относительный, он считается относительно родительского каталога текущего файла.
    /// После объединения путь нормализуется, убирая '.' и '..'.
    /// Если новый путь указывает на несуществующие директории, они будут созданы.
    ///
    /// # Пример
    /// 
    ///  Исходный путь: "/home/user/Downloads/abc/file.txt"
    /// ```rust
    /// let mut file = BFile::new("/home/user/Downloads/abc/file.txt").expect("не удалось создать файл");
    /// file.write("").expect("не удалось записать файл");
    ///
    /// println!("File path: {:?}", file.path());
    /// ```
    ///  Вывод: "/home/user/Downloads/abc/file.txt"
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

    /// Возвращает текущий путь файла.
    ///
    /// # Пример
    ///
    /// ```rust
    /// println!("Путь файла: {:?}", file.path());
    /// ```
    pub fn path(&self) -> &Path {
        &self.path
    }
}



/// Нормализует путь, учитывая './' и '../'
///
/// # Пример
///
/// ```rust
/// let normalized = normalize_path("/home/user/Downloads/directory/../file.txt");
/// assert_eq!(normalized.to_str().unwrap(), "/home/user/directory/file.txt");
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
