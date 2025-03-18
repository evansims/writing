use common_errors::Result;
use common_fs::{create_dir, dir_exists, delete_dir, file_exists, write_file};
use std::fs;
use std::path::{Path, PathBuf};
use tempfile::tempdir;

#[test]
fn test_directory_operations_basic() -> Result<()> {
    // Create a temp directory for testing
    let temp_dir = tempdir()?;
    let test_dir_path = temp_dir.path().join("test_dir");

    // Test create_dir
    create_dir(&test_dir_path)?;
    assert!(dir_exists(&test_dir_path));

    // Test delete_dir
    delete_dir(&test_dir_path)?;
    assert!(!dir_exists(&test_dir_path));

    Ok(())
}

#[test]
fn test_directory_operations_nested() -> Result<()> {
    // Create a temp directory for testing
    let temp_dir = tempdir()?;
    let nested_dir_path = temp_dir.path().join("parent/child/grandchild");

    // Test creating nested directories
    create_dir(&nested_dir_path)?;
    assert!(dir_exists(&nested_dir_path));
    assert!(dir_exists(&temp_dir.path().join("parent/child")));
    assert!(dir_exists(&temp_dir.path().join("parent")));

    // Test deleting parent directory removes all children
    delete_dir(&temp_dir.path().join("parent"))?;
    assert!(!dir_exists(&nested_dir_path));
    assert!(!dir_exists(&temp_dir.path().join("parent/child")));
    assert!(!dir_exists(&temp_dir.path().join("parent")));

    Ok(())
}

#[test]
fn test_directory_with_content() -> Result<()> {
    // Create a temp directory for testing
    let temp_dir = tempdir()?;
    let test_dir_path = temp_dir.path().join("dir_with_content");

    // Create the directory
    create_dir(&test_dir_path)?;

    // Add some files to the directory
    let file1_path = test_dir_path.join("file1.txt");
    let file2_path = test_dir_path.join("file2.txt");

    write_file(&file1_path, "Content of file 1")?;
    write_file(&file2_path, "Content of file 2")?;

    // Verify files exist
    assert!(file_exists(&file1_path));
    assert!(file_exists(&file2_path));

    // Create a subdirectory with a file
    let subdir_path = test_dir_path.join("subdir");
    create_dir(&subdir_path)?;

    let subdir_file_path = subdir_path.join("subfile.txt");
    write_file(&subdir_file_path, "Content of subfile")?;

    // Delete the parent directory
    delete_dir(&test_dir_path)?;

    // Verify everything is gone
    assert!(!dir_exists(&test_dir_path));
    assert!(!file_exists(&file1_path));
    assert!(!file_exists(&file2_path));
    assert!(!dir_exists(&subdir_path));
    assert!(!file_exists(&subdir_file_path));

    Ok(())
}

#[test]
fn test_directory_operations_edge_cases() -> Result<()> {
    // Create a temp directory for testing
    let temp_dir = tempdir()?;

    // Test with directory name containing special characters
    let special_dir_path = temp_dir.path().join("special-dir!@#$%");
    create_dir(&special_dir_path)?;
    assert!(dir_exists(&special_dir_path));

    // Test with empty directory name (should be current directory, which already exists)
    let empty_dir_path = temp_dir.path().join("");
    create_dir(&empty_dir_path)?; // Should succeed because the parent already exists

    // Test with very long directory path (close to PATH_MAX)
    // Most systems have PATH_MAX around 4096, so we'll make something reasonably long but not too extreme
    let long_segment = "a".repeat(50);
    let mut long_path = temp_dir.path().to_path_buf();

    // Create a path with 10 segments of 50 chars each
    for _ in 0..10 {
        long_path = long_path.join(&long_segment);
    }

    create_dir(&long_path)?;
    assert!(dir_exists(&long_path));

    // Clean up
    delete_dir(&special_dir_path)?;
    delete_dir(&long_path.parent().unwrap())?; // Delete parent to clean up the long path

    Ok(())
}

#[test]
fn test_delete_nonexistent_directory() {
    // Create a temp directory for testing
    let temp_dir = tempdir().unwrap();
    let nonexistent_dir = temp_dir.path().join("nonexistent_dir");

    // Deleting a non-existent directory should fail
    let result = delete_dir(&nonexistent_dir);
    assert!(result.is_err());
}