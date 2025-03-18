use common_errors::Result;
use common_fs::{read_file, write_file, file_exists, delete_file, create_dir};
use std::fs;
use std::path::Path;
use tempfile::tempdir;

#[test]
fn test_file_operations_basic() -> Result<()> {
    // Create a temp directory for testing
    let temp_dir = tempdir()?;
    let test_path = temp_dir.path().join("test_file.txt");
    let test_content = "Hello, world!";

    // Test write_file
    write_file(&test_path, test_content)?;
    assert!(file_exists(&test_path));

    // Test read_file
    let read_content = read_file(&test_path)?;
    assert_eq!(read_content, test_content);

    // Test delete_file
    delete_file(&test_path)?;
    assert!(!file_exists(&test_path));

    Ok(())
}

#[test]
fn test_file_operations_with_unicode() -> Result<()> {
    // Create a temp directory for testing
    let temp_dir = tempdir()?;
    let test_path = temp_dir.path().join("unicode_file.txt");
    let test_content = "こんにちは世界! Здравствуй, мир! مرحبا بالعالم!";

    // Test write_file with Unicode content
    write_file(&test_path, test_content)?;
    assert!(file_exists(&test_path));

    // Test read_file with Unicode content
    let read_content = read_file(&test_path)?;
    assert_eq!(read_content, test_content);

    // Clean up
    delete_file(&test_path)?;

    Ok(())
}

#[test]
fn test_file_operations_with_nested_paths() -> Result<()> {
    // Create a temp directory for testing
    let temp_dir = tempdir()?;
    let nested_dir = temp_dir.path().join("nested/directory/path");
    create_dir(&nested_dir)?;

    let test_path = nested_dir.join("nested_file.txt");
    let test_content = "Content in nested file";

    // Test write_file to nested path
    write_file(&test_path, test_content)?;
    assert!(file_exists(&test_path));

    // Test read_file from nested path
    let read_content = read_file(&test_path)?;
    assert_eq!(read_content, test_content);

    // Clean up
    delete_file(&test_path)?;

    Ok(())
}

#[test]
fn test_file_operations_edge_cases() -> Result<()> {
    // Create a temp directory for testing
    let temp_dir = tempdir()?;

    // Test with empty file
    let empty_file_path = temp_dir.path().join("empty.txt");
    write_file(&empty_file_path, "")?;
    assert!(file_exists(&empty_file_path));
    assert_eq!(read_file(&empty_file_path)?, "");

    // Test with very large content (100KB)
    let large_file_path = temp_dir.path().join("large.txt");
    let large_content = "a".repeat(100 * 1024); // 100KB of 'a's
    write_file(&large_file_path, &large_content)?;
    assert_eq!(read_file(&large_file_path)?.len(), large_content.len());

    // Test with file with special characters in name
    let special_name_path = temp_dir.path().join("file-with-special_chars!@#$%.txt");
    write_file(&special_name_path, "Special name content")?;
    assert!(file_exists(&special_name_path));
    assert_eq!(read_file(&special_name_path)?, "Special name content");

    // Clean up
    delete_file(&empty_file_path)?;
    delete_file(&large_file_path)?;
    delete_file(&special_name_path)?;

    Ok(())
}

#[test]
fn test_delete_nonexistent_file() -> Result<()> {
    // Create a temp directory for testing
    let temp_dir = tempdir()?;
    let nonexistent_path = temp_dir.path().join("nonexistent.txt");

    // Deleting a non-existent file should succeed
    delete_file(&nonexistent_path)?;

    Ok(())
}