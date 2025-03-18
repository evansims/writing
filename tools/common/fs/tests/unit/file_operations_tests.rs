use common_errors::Result;
use common_fs::{read_file, write_file, delete_file, path_exists};
use common_test_utils::FileSystemFixture;
use std::fs;
use std::path::Path;

#[test]
fn test_file_operations_basic() -> Result<()> {
    // Create a temp directory for testing
    let fixture = FileSystemFixture::new()?;

    // Test file path
    let test_path = fixture.abs_path("test_file.txt");
    let test_content = "Hello, world!";

    // Test write_file
    write_file(&test_path, test_content)?;
    assert!(fixture.file_exists("test_file.txt"));

    // Test read_file
    let read_content = read_file(&test_path)?;
    assert_eq!(read_content, test_content);

    // Test file_exists
    assert!(path_exists(&test_path));

    // Test delete_file
    delete_file(&test_path)?;
    assert!(!fixture.file_exists("test_file.txt"));

    Ok(())
}

#[test]
fn test_file_operations_with_unicode() -> Result<()> {
    // Create a temp directory for testing
    let fixture = FileSystemFixture::new()?;

    // Test file path with a unicode filename
    let test_path = fixture.abs_path("unicode_file_🚀.txt");
    let test_content = "Unicode content: 你好，世界！";

    // Test write_file with unicode content
    write_file(&test_path, test_content)?;

    // Test read_file with unicode content
    let read_content = read_file(&test_path)?;
    assert_eq!(read_content, test_content);

    // Clean up
    delete_file(&test_path)?;

    Ok(())
}

#[test]
fn test_file_operations_with_nested_paths() -> Result<()> {
    // Create a temp directory for testing
    let fixture = FileSystemFixture::new()?;

    // Create nested path
    let nested_path = fixture.abs_path("nested/directories/for/testing/file.txt");
    let nested_dir = Path::new(&nested_path).parent().unwrap();
    let test_content = "Content in nested directory";

    // Create the parent directories
    fs::create_dir_all(nested_dir)?;

    // Test writing to a file in a nested directory
    write_file(&nested_path, test_content)?;

    // Verify directories were created
    assert!(fixture.dir_exists("nested"));
    assert!(fixture.dir_exists("nested/directories"));
    assert!(fixture.dir_exists("nested/directories/for"));
    assert!(fixture.dir_exists("nested/directories/for/testing"));

    // Verify file was created
    assert!(fixture.file_exists("nested/directories/for/testing/file.txt"));

    // Verify content
    let read_content = read_file(&nested_path)?;
    assert_eq!(read_content, test_content);

    Ok(())
}

#[test]
fn test_file_operations_edge_cases() -> Result<()> {
    // Create a temp directory for testing
    let fixture = FileSystemFixture::new()?;

    // Test with empty file
    let empty_file_path = fixture.abs_path("empty_file.txt");
    write_file(&empty_file_path, "")?;
    assert!(fixture.file_exists("empty_file.txt"));
    let content = read_file(&empty_file_path)?;
    assert_eq!(content, "");

    // Test with large file (1MB)
    let large_file_path = fixture.abs_path("large_file.txt");
    let large_content = "a".repeat(1024 * 1024); // 1MB of 'a's
    write_file(&large_file_path, &large_content)?;
    assert!(fixture.file_exists("large_file.txt"));
    let read_large_content = read_file(&large_file_path)?;
    assert_eq!(read_large_content.len(), large_content.len());

    // Test with binary content
    let binary_file_path = fixture.abs_path("binary_file.bin");
    let binary_content = (0..255).map(|b| b as u8).collect::<Vec<u8>>();
    fs::write(&binary_file_path, &binary_content)?;
    assert!(Path::new(&binary_file_path).exists());

    // Test reading a file that doesn't exist
    let nonexistent_path = fixture.abs_path("nonexistent_file.txt");
    let result = read_file(&nonexistent_path);
    assert!(result.is_err());

    // Test deleting a file that doesn't exist
    let result = delete_file(&nonexistent_path);
    // The delete_file function doesn't return an error if the file doesn't exist
    assert!(result.is_ok());

    Ok(())
}

#[test]
fn test_file_exists() {
    // Create a temp directory for testing
    let fixture = FileSystemFixture::new().unwrap();

    // Test file path
    let test_path = fixture.abs_path("test_exists.txt");
    let test_content = "Testing file_exists";

    // Before creating the file
    assert!(!path_exists(&test_path));

    // After creating the file
    write_file(&test_path, test_content).unwrap();
    assert!(path_exists(&test_path));

    // After deleting the file
    delete_file(&test_path).unwrap();
    assert!(!path_exists(&test_path));

    // With a directory
    let dir_path = fixture.abs_path("test_dir");
    fs::create_dir(&dir_path).unwrap();
    assert!(path_exists(&dir_path)); // Should return true for directories
}