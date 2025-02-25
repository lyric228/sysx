use tempfile::NamedTempFile;
use std::path::PathBuf;
use sysx::io::fs::*;


#[test]
fn test_file_operations() -> std::io::Result<()> {
    let temp_file = NamedTempFile::new()?;
    let path = temp_file.path();
    
    let file = BFile::new(path)?;
    file.write("content")?;
    assert_eq!(file.read()?, "content");
    
    file.delete()?;
    assert!(!file.exists());
    
    Ok(())
}

#[test]
fn test_path_operations() {
    let normalized = normalize_path("/home/../user/./file.txt");
    assert_eq!(normalized, PathBuf::from("/user/file.txt"));
}
