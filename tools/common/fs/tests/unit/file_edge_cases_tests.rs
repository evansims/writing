use common_errors::{Result, WritingError};
use common_fs::{read_file, write_file, delete_file, path_exists, file::*};
use common_test_utils::FileSystemFixture;
use std::path::Path;
use std::fs;
use std::sync::Arc;
use mockall::predicate::*;
use common_test_utils::mocks::MockFileSystem;

#[test]
fn test_file_read_nonexistent() {
    // Arrange
    let fixture = FileSystemFixture::new().unwrap();
    let non_existent_file = fixture.abs_path("does_not_exist.txt");

    // Act
    let result = read_file(&non_existent_file);

    // Assert
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.to_string().contains("not found") || err.to_string().contains("No such file"));
}

#[test]
fn test_file_permissions() -> Result<()> {
    // Skip on non-unix platforms
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;

        // Arrange
        let fixture = FileSystemFixture::new()?;
        let test_file = fixture.abs_path("read_only.txt");

        // Create file
        write_file(&test_file, "test content")?;

        // Make it read-only
        let metadata = fs::metadata(&test_file)?;
        let mut perms = metadata.permissions();
        perms.set_mode(0o444); // Read-only for all
        fs::set_permissions(&test_file, perms)?;

        // Act
        let read_result = read_file(&test_file);
        let write_result = write_file(&test_file, "new content");

        // Reset permissions for cleanup
        let mut perms = fs::metadata(&test_file)?.permissions();
        perms.set_mode(0o644); // Read-write for owner
        fs::set_permissions(&test_file, perms)?;

        // Assert
        assert!(read_result.is_ok()); // Should still be able to read
        assert!(write_result.is_err()); // Should fail to write
        assert!(write_result.unwrap_err().to_string().contains("permission denied"));
    }

    Ok(())
}

#[test]
fn test_file_with_special_characters() -> Result<()> {
    // Arrange
    let fixture = FileSystemFixture::new()?;
    let special_chars = fixture.abs_path("special_@#$%^&*()_+{}\"|:?><.txt");
    let content = "Content with special characters: !@#$%^&*()_+{}\":|<>?";

    // Act
    write_file(&special_chars, content)?;
    let read_content = read_file(&special_chars)?;

    // Assert
    assert_eq!(read_content, content);

    // Cleanup
    delete_file(&special_chars)?;

    Ok(())
}

#[test]
fn test_file_with_extreme_size() -> Result<()> {
    // Arrange
    let fixture = FileSystemFixture::new()?;
    let large_file = fixture.abs_path("large_file.txt");

    // Create content that's large but not too large for tests (1MB)
    let large_content = "A".repeat(1_000_000);

    // Act & Assert - Large file
    write_file(&large_file, &large_content)?;
    let read_large = read_file(&large_file)?;
    assert_eq!(read_large.len(), large_content.len());

    // Empty file
    let empty_file = fixture.abs_path("empty_file.txt");
    write_file(&empty_file, "")?;
    let read_empty = read_file(&empty_file)?;
    assert_eq!(read_empty, "");

    // Cleanup
    delete_file(&large_file)?;
    delete_file(&empty_file)?;

    Ok(())
}

#[test]
fn test_concurrent_file_operations() -> Result<()> {
    use std::thread;

    // Arrange
    let fixture = FileSystemFixture::new()?;
    let test_file = fixture.abs_path("concurrent.txt");

    // Initial content
    write_file(&test_file, "initial content")?;

    // Create 10 threads that try to read and write
    let mut handles = vec![];
    for i in 0..10 {
        let file_path = test_file.clone();
        let handle = thread::spawn(move || {
            // Read file
            let _ = read_file(&file_path);

            // Write to file
            let _ = write_file(&file_path, &format!("Thread {} content", i));

            // Small delay to increase chance of race conditions
            thread::sleep(std::time::Duration::from_millis(5));
        });
        handles.push(handle);
    }

    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }

    // Verify file exists and can be read after concurrent operations
    let final_content = read_file(&test_file)?;
    assert!(!final_content.is_empty());

    // Cleanup
    delete_file(&test_file)?;

    Ok(())
}

#[test]
fn test_symlink_handling() -> Result<()> {
    // Skip on platforms where symlinks are problematic
    #[cfg(not(windows))]
    {
        // Arrange
        let fixture = FileSystemFixture::new()?;
        let original_file = fixture.abs_path("original.txt");
        let symlink_file = fixture.abs_path("symlink.txt");

        // Create original file
        write_file(&original_file, "original content")?;

        // Create symlink
        std::os::unix::fs::symlink(&original_file, &symlink_file)?;

        // Act
        let symlink_content = read_file(&symlink_file)?;

        // Assert
        assert_eq!(symlink_content, "original content");

        // Modify through symlink
        write_file(&symlink_file, "modified content")?;
        let original_content = read_file(&original_file)?;
        assert_eq!(original_content, "modified content");

        // Delete symlink
        delete_file(&symlink_file)?;
        assert!(!path_exists(&symlink_file));
        assert!(path_exists(&original_file));

        // Cleanup
        delete_file(&original_file)?;
    }

    Ok(())
}

#[test]
fn test_file_locking() -> Result<()> {
    // This test checks how the file system handles concurrent access with locks

    #[cfg(unix)]
    {
        use std::fs::OpenOptions;
        use std::io::Write;
        use std::os::unix::fs::OpenOptionsExt;

        // Arrange
        let fixture = FileSystemFixture::new()?;
        let lock_file = fixture.abs_path("locked.txt");

        // Create and write to file
        write_file(&lock_file, "initial content")?;

        // Open file with exclusive lock
        let mut exclusive_file = OpenOptions::new()
            .write(true)
            .custom_flags(libc::O_EXLOCK) // BSD-specific exclusive lock
            .open(&lock_file)?;

        // Try to write to locked file using our utility
        let write_result = write_file(&lock_file, "new content");

        // Release lock
        exclusive_file.write_all(b"from locked handle")?;
        drop(exclusive_file);

        // Assert
        // On some systems this might succeed by waiting, on others it might fail
        // We just want to ensure it doesn't crash or hang indefinitely
        if write_result.is_err() {
            println!("Lock error (expected): {}", write_result.unwrap_err());
        }

        // After lock release, we should be able to write
        write_file(&lock_file, "final content")?;
        let final_content = read_file(&lock_file)?;
        assert!(final_content == "final content" || final_content == "from locked handle");

        // Cleanup
        delete_file(&lock_file)?;
    }

    Ok(())
}