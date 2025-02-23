use std::path::PathBuf;
use std::io::Write;

use tempfile::NamedTempFile;
use sysx::io::fs::*;


// BFile::new tests

#[test]
// Тест создания нового BFile с абсолютным путем.
// Проверяет корректность инициализации с абсолютным путем.
fn test_bfile_new_absolute_path() {
    let path = std::env::current_dir().unwrap().join("test.txt");
    let file = BFile::new(&path).unwrap();
    assert_eq!(file.path(), path);
}

#[test]
// Тест создания нового BFile с относительным путем.
// Проверяет преобразование относительного пути в абсолютный.
fn test_bfile_new_relative_path() {
    let file = BFile::new("test.txt").unwrap();
    assert!(file.path().is_absolute());
}


// BFile operations tests

#[test]
// Тест основных операций с файлом: запись, чтение, проверка существования.
fn test_basic_file_operations() -> std::io::Result<()> {
    let temp_file = NamedTempFile::new()?;
    let mut bfile = BFile::new(temp_file.path())?;
    
    // Проверка записи и чтения
    bfile.write("test content")?;
    assert_eq!(bfile.read()?, "test content");
    
    // Проверка существования
    assert!(bfile.exists());
    
    // Проверка удаления
    bfile.delete()?;
    assert!(!bfile.exists());
    
    Ok(())
}

#[test]
// Тест операции добавления в файл.
// Проверяет корректность добавления контента в конец файла.
fn test_file_append() -> std::io::Result<()> {
    let temp_file = NamedTempFile::new()?;
    let bfile = BFile::new(temp_file.path())?;
    
    bfile.write("first line\n")?;
    bfile.append("second line")?;
    
    assert_eq!(bfile.read()?, "first line\nsecond line");
    Ok(())
}


// BFile rename tests

#[test]
// Тест переименования файла.
// Проверяет корректность изменения пути файла после переименования.
fn test_file_rename() -> std::io::Result<()> {
    let temp_file = NamedTempFile::new()?;
    let mut bfile = BFile::new(temp_file.path())?;
    let new_path = temp_file.path().with_file_name("renamed.txt");
    
    bfile.rename(&new_path)?;
    assert_eq!(bfile.path(), new_path);
    
    Ok(())
}


// normalize_path tests

#[test]
// Тест нормализации пути с компонентами "." и "..".
// Проверяет удаление избыточных компонентов пути.
fn test_normalize_path() {
    let path = "./dir/../dir/./file.txt";
    let normalized = normalize_path(path);
    assert_eq!(normalized, PathBuf::from("dir/file.txt"));
}

#[test]
// Тест нормализации абсолютного пути.
// Проверяет сохранение корректности абсолютного пути.
fn test_normalize_absolute_path() {
    let path = "/home/user/../user/./file.txt";
    let normalized = normalize_path(path);
    assert_eq!(normalized, PathBuf::from("/home/user/file.txt"));
}


// get_dir_size tests

#[test]
// Тест подсчета размера пустой директории.
// Проверяет корректность подсчета для пустой директории.
fn test_empty_dir_size() -> std::io::Result<()> {
    let temp_dir = tempfile::tempdir()?;
    let size = get_dir_size(temp_dir.path().to_str().unwrap())?;
    assert_eq!(size, 0);
    Ok(())
}

#[test]
// Тест подсчета размера директории с файлами.
// Проверяет корректность подсчета общего размера файлов.
fn test_dir_size_with_files() -> std::io::Result<()> {
    let temp_dir = tempfile::tempdir()?;
    let file_path = temp_dir.path().join("test.txt");
    std::fs::write(&file_path, "test content")?;
    
    let size = get_dir_size(temp_dir.path().to_str().unwrap())?;
    assert_eq!(size, 12); // "test content" = 12 bytes
    Ok(())
}
