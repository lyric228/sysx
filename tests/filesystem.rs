use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use sysx::io::fs::File;
use tempfile::tempdir;

#[test]
fn test_file_create_read_write() -> std::io::Result<()> {
    let dir = tempdir()?;
    let test_path = dir.path().join("test_file.txt");
    println!(
        "Test file path (create_read_write): {}",
        test_path.display()
    );

    let nested_path = dir.path().join("nested/dirs/test.txt");
    println!(
        "Nested file path (create_read_write): {}",
        nested_path.display()
    );
    File::create(&nested_path)?;
    assert!(nested_path.exists());

    let mut write_file = File::create(&test_path)?;
    write_file.write_all(b"Hello, world!")?;
    assert!(test_path.exists());

    let mut read_file = File::open(&test_path)?;
    let content = read_file.read_to_string()?;
    assert_eq!(content, "Hello, world!");

    let mut append_file = File::append(&test_path)?;
    append_file.write_all(b" More text.")?;
    let mut updated_read_file = File::open(&test_path)?;
    let updated_content = updated_read_file.read_to_string()?;
    assert_eq!(updated_content, "Hello, world! More text.");
    Ok(())
}

#[test]
fn test_file_rename_delete() -> std::io::Result<()> {
    let dir = tempdir()?;
    let original_path = dir.path().join("original.txt");
    let new_path = dir.path().join("renamed.txt");
    let mut file = File::create(&original_path)?;
    file.write_all(b"Test content")?;
    assert!(original_path.exists());

    fs::rename(&original_path, &new_path)?;
    assert!(!original_path.exists());
    assert!(new_path.exists());

    let mut read_file = File::open(&new_path)?;
    let content = read_file.read_to_string()?;
    assert_eq!(content, "Test content");

    fs::remove_file(&new_path)?;
    assert!(!new_path.exists());

    fs::remove_file(&new_path).ok();
    Ok(())
}

#[test]
fn test_metadata() -> std::io::Result<()> {
    let dir = tempdir()?;
    let path = dir.path().join("metadata_test.txt");
    let mut file = File::create(&path)?;
    file.write_all(b"test")?;
    let metadata = fs::metadata(&path)?;

    assert!(!metadata.is_dir());
    assert!(metadata.is_file());
    assert_eq!(metadata.len(), 4);

    Ok(())
}

fn normalize_path<P: AsRef<Path>>(path: P) -> PathBuf {
    let mut components = path.as_ref().components().peekable();
    let mut pathbuf = PathBuf::new();
    while let Some(component) = components.next() {
        match component {
            std::path::Component::CurDir => {}
            std::path::Component::ParentDir => {
                pathbuf.pop();
            }
            _ => {
                pathbuf.push(component);
            }
        }
    }
    pathbuf
}

#[test]
fn test_normalize_path() {
    let normalized = normalize_path("./docs/../config/./file.txt");
    assert_eq!(normalized.file_name().unwrap().to_str(), Some("file.txt"));
    assert!(normalized.ends_with("config/file.txt"));

    let normalized = normalize_path("/home/../usr/./local");
    assert_eq!(normalized, PathBuf::from("/usr/local"));

    let normalized = normalize_path("a/b/c/../../d");
    assert_eq!(normalized.file_name().unwrap().to_str(), Some("d"));
    assert!(normalized.ends_with("a/d"));
}

fn dir_size<P: AsRef<Path>>(path: P) -> io::Result<u64> {
    let mut total_size = 0;
    println!("Calculating dir_size for: {}", path.as_ref().display());
    for entry in fs::read_dir(path.as_ref())? {
        let entry = entry?;
        let entry_path = entry.path();
        let metadata = fs::metadata(&entry_path)?;
        println!(
            "  Entry: {}, size: {}, is_dir: {}",
            entry_path.display(),
            metadata.len(),
            metadata.is_dir()
        );
        if !metadata.is_dir() {
            total_size += metadata.len();
        } else if metadata.is_dir() {
            total_size += dir_size(entry_path)?;
        }
    }
    println!("Total size for {}: {}", path.as_ref().display(), total_size);
    Ok(total_size)
}

#[test]
fn test_dir_size() -> std::io::Result<()> {
    let dir = tempdir()?;
    println!("Temp dir path for dir_size: {}", dir.path().display());

    let file1_path = dir.path().join("file1.txt");
    println!("File1 path: {}", file1_path.display());
    let mut file1 = File::create(file1_path)?;
    file1.write_all(b"1234")?;

    let file2_path = dir.path().join("file2.txt");
    println!("File2 path: {}", file2_path.display());
    let mut file2 = File::create(file2_path)?;
    file2.write_all(b"56789")?;

    let subdir_path = dir.path().join("subdir");
    println!("Subdir path: {}", subdir_path.display());
    fs::create_dir(&subdir_path)?;
    let file3_path = subdir_path.join("file3.txt");
    println!("File3 path: {}", file3_path.display());
    let mut file3 = File::create(file3_path)?;
    file3.write_all(b"abcdef")?;

    let size = dir_size(dir.path())?;
    assert_eq!(size, 15);

    Ok(())
}

#[test]
fn test_complex_scenario() -> std::io::Result<()> {
    let dir = tempdir()?;
    println!(
        "Temp dir path for complex_scenario: {}",
        dir.path().display()
    );

    let path1 = dir.path().join("level1/level2/file1.txt");
    println!("Path1: {}", path1.display());
    let path2 = dir.path().join("level1/file2.txt");
    println!("Path2: {}", path2.display());
    let new_path = dir.path().join("new_location/renamed.txt");
    println!("New Path: {}", new_path.display());

    if let Some(new_dir_path) = new_path.parent() {
        println!("Creating directory: {}", new_dir_path.display());
        fs::create_dir_all(new_dir_path)?;
    }

    {
        println!("Creating file: {}", path1.display());
        let mut file = File::create(&path1)?;
        file.write_all(b"Nested file content")?;
    }

    {
        println!("Creating file: {}", path2.display());
        let mut file_level1 = File::create(&path2)?;
        file_level1.write_all(b"Level 1 content")?;
    }

    assert!(path1.exists());
    assert!(path2.exists());

    {
        println!("Opening file for read: {}", path1.display());
        let mut read_file1 = File::open(&path1)?;
        assert_eq!(read_file1.read_to_string()?, "Nested file content");
    }

    println!("Renaming {} to {}", path2.display(), new_path.display());
    fs::rename(&path2, &new_path)?;
    println!(
        "Checking if new path exists after rename: {}",
        new_path.display()
    );
    assert!(new_path.exists());

    assert!(!path2.exists());
    assert!(new_path.exists());

    {
        println!("Opening file after rename: {}", new_path.display());
        let mut read_new_path = File::open(&new_path)?;
        assert_eq!(read_new_path.read_to_string()?, "Level 1 content");
    }

    let size = dir_size(dir.path())?;
    assert_eq!(
        size,
        "Nested file content".len() as u64 + "Level 1 content".len() as u64
    );

    Ok(())
}
