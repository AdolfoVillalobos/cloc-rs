use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use tempfile::TempDir;


use cloc_rs::{count_lines, count_lines_in_repo};


#[test]
fn test_count_lines() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.txt");
    let mut file = File::create(&file_path).unwrap();
    writeln!(file, "Line 1\nLine 2\nLine 3").unwrap();

    assert_eq!(count_lines(&file_path).unwrap(), 3);
}

#[test]
fn test_count_lines_in_repo() {
    let temp_dir = TempDir::new().unwrap();
    
    // Create test files
    let file1 = temp_dir.path().join("file1.txt");
    let file2 = temp_dir.path().join("file2.rs");
    let file3 = temp_dir.path().join("subdir").join("file3.txt");

    File::create(&file1).unwrap().write_all(b"Line 1\nLine 2").unwrap();
    File::create(&file2).unwrap().write_all(b"Line 1\nLine 2\nLine 3").unwrap();
    fs::create_dir_all(temp_dir.path().join("subdir")).unwrap();
    File::create(&file3).unwrap().write_all(b"Line 1").unwrap();

    // Test without extension filter
    assert_eq!(count_lines_in_repo(temp_dir.path(), None).unwrap(), 6);

    // Test with extension filter
    assert_eq!(count_lines_in_repo(temp_dir.path(), Some("txt")).unwrap(), 3);
}