use common_errors::{Result, WritingError};
use common_fs::{
    create_dir_all, delete_dir_all, path_exists, copy_dir_all, move_dir,
    directory::*,
};
use common_test_utils::FileSystemFixture;
use std::path::Path;
use std::fs;

#[test]
fn test_create_directories_with_extreme_depth() -> Result<()> {
    // Arrange
    let fixture = FileSystemFixture::new()?;

    // Create directory with extreme depth (within reason for tests)
    let mut deep_path = fixture.abs_path("depth").to_string_lossy().to_string();
    for i in 0..30 {
        deep_path = format!("{}/level_{}", deep_path, i);
    }

    // Act
    create_dir_all(&deep_path)?;

    // Assert
    assert!(path_exists(&deep_path));

    // Cleanup
    delete_dir_all(&fixture.abs_path("depth"))?;

    Ok(())
}

#[test]
fn test_create_directory_with_special_characters() -> Result<()> {
    // Arrange
    let fixture = FileSystemFixture::new()?;

    // Create path with special characters (that are valid in filenames)
    let special_dir = fixture.abs_path("special_@#$%^&()_+{}|;.dir");

    // Act
    create_dir_all(&special_dir)?;

    // Assert
    assert!(path_exists(&special_dir));

    // Cleanup
    delete_dir_all(&special_dir)?;

    Ok(())
}

#[test]
fn test_copy_directory_to_existing_target() -> Result<()> {
    // Arrange
    let fixture = FileSystemFixture::new()?;

    // Create source directory with files
    let source_dir = fixture.abs_path("source_dir");
    create_dir_all(&source_dir)?;
    fs::write(Path::new(&source_dir).join("file1.txt"), "content1")?;
    fs::write(Path::new(&source_dir).join("file2.txt"), "content2")?;

    // Create target directory that already exists
    let target_dir = fixture.abs_path("target_dir");
    create_dir_all(&target_dir)?;
    fs::write(Path::new(&target_dir).join("existing.txt"), "existing")?;

    // Act - Copy directory to existing target
    copy_dir_all(&source_dir, &target_dir)?;

    // Assert - Both source files should be copied and existing file preserved
    assert!(Path::new(&target_dir).join("file1.txt").exists());
    assert!(Path::new(&target_dir).join("file2.txt").exists());
    assert!(Path::new(&target_dir).join("existing.txt").exists());

    // Cleanup
    delete_dir_all(&source_dir)?;
    delete_dir_all(&target_dir)?;

    Ok(())
}

#[test]
fn test_copy_directory_with_symlinks() -> Result<()> {
    // Skip on platforms where symlinks are problematic
    #[cfg(not(windows))]
    {
        // Arrange
        let fixture = FileSystemFixture::new()?;

        // Create source directory with files and symlink
        let source_dir = fixture.abs_path("source_with_symlinks");
        let external_dir = fixture.abs_path("external_dir");
        create_dir_all(&source_dir)?;
        create_dir_all(&external_dir)?;

        // Create a file in external dir and a symlink to it in source
        fs::write(Path::new(&external_dir).join("external.txt"), "external content")?;
        std::os::unix::fs::symlink(
            Path::new(&external_dir).join("external.txt"),
            Path::new(&source_dir).join("symlink.txt")
        )?;

        // Also create a regular file
        fs::write(Path::new(&source_dir).join("regular.txt"), "regular content")?;

        // Act - Copy directory with symlink
        let target_dir = fixture.abs_path("target_with_symlinks");
        copy_dir_all(&source_dir, &target_dir)?;

        // Assert
        // Symlink should be copied as a regular file with same content
        assert!(Path::new(&target_dir).join("symlink.txt").exists());
        let symlink_content = fs::read_to_string(Path::new(&target_dir).join("symlink.txt"))?;
        assert_eq!(symlink_content, "external content");

        // Regular file should be copied normally
        assert!(Path::new(&target_dir).join("regular.txt").exists());

        // Cleanup
        delete_dir_all(&source_dir)?;
        delete_dir_all(&target_dir)?;
        delete_dir_all(&external_dir)?;
    }

    Ok(())
}

#[test]
fn test_move_directory_overwriting_target() -> Result<()> {
    // Arrange
    let fixture = FileSystemFixture::new()?;

    // Create source directory with files
    let source_dir = fixture.abs_path("source_to_move");
    create_dir_all(&source_dir)?;
    fs::write(Path::new(&source_dir).join("file1.txt"), "content1")?;
    fs::write(Path::new(&source_dir).join("file2.txt"), "content2")?;

    // Create target directory that already exists with conflicting file
    let target_dir = fixture.abs_path("existing_target");
    create_dir_all(&target_dir)?;
    fs::write(Path::new(&target_dir).join("file1.txt"), "old content")?;
    fs::write(Path::new(&target_dir).join("existing.txt"), "existing")?;

    // Act - Move directory to existing target
    move_dir(&source_dir, &target_dir)?;

    // Assert
    // Source should not exist anymore
    assert!(!Path::new(&source_dir).exists());

    // Target should contain the moved files (overwriting conflicting ones)
    assert!(Path::new(&target_dir).join("file1.txt").exists());
    assert!(Path::new(&target_dir).join("file2.txt").exists());
    assert!(Path::new(&target_dir).join("existing.txt").exists());

    // Check content of overwritten file
    let file1_content = fs::read_to_string(Path::new(&target_dir).join("file1.txt"))?;
    assert_eq!(file1_content, "content1");

    // Cleanup
    delete_dir_all(&target_dir)?;

    Ok(())
}

#[test]
fn test_directory_permissions() -> Result<()> {
    // Skip on non-unix platforms
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;

        // Arrange
        let fixture = FileSystemFixture::new()?;
        let test_dir = fixture.abs_path("permission_dir");

        // Create directory
        create_dir_all(&test_dir)?;

        // Make it read-only
        let metadata = fs::metadata(&test_dir)?;
        let mut perms = metadata.permissions();
        perms.set_mode(0o555); // r-xr-xr-x (read and execute only)
        fs::set_permissions(&test_dir, perms)?;

        // Act - Try to create a file inside the read-only directory
        let result = fs::write(Path::new(&test_dir).join("test.txt"), "test");

        // Reset permissions for cleanup
        let mut perms = fs::metadata(&test_dir)?.permissions();
        perms.set_mode(0o755); // rwxr-xr-x (restore write permission)
        fs::set_permissions(&test_dir, perms)?;

        // Assert
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("permission denied"));

        // Cleanup
        delete_dir_all(&test_dir)?;
    }

    Ok(())
}

#[test]
fn test_empty_directory_handling() -> Result<()> {
    // Arrange
    let fixture = FileSystemFixture::new()?;

    // Create empty directories
    let empty_dir1 = fixture.abs_path("empty1");
    let empty_dir2 = fixture.abs_path("empty2");
    create_dir_all(&empty_dir1)?;
    create_dir_all(&empty_dir2)?;

    // Act

    // Check if directory is empty (should be true)
    let empty1_result = !has_content(&empty_dir1)?;

    // Create a file in the second directory
    fs::write(Path::new(&empty_dir2).join("file.txt"), "content")?;
    let empty2_result = !has_content(&empty_dir2)?;

    // Assert
    assert!(empty1_result, "Directory should be considered empty");
    assert!(!empty2_result, "Directory with files should not be considered empty");

    // Cleanup
    delete_dir_all(&empty_dir1)?;
    delete_dir_all(&empty_dir2)?;

    Ok(())
}

#[test]
fn test_moving_nonexistent_directory() {
    // Arrange
    let fixture = FileSystemFixture::new().unwrap();
    let source_dir = fixture.abs_path("does_not_exist");
    let target_dir = fixture.abs_path("target_dir");

    // Act
    let result = move_dir(&source_dir, &target_dir);

    // Assert
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.to_string().contains("not found") || err.to_string().contains("No such file"));
}

#[test]
fn test_concurrent_directory_operations() -> Result<()> {
    use std::thread;
    use std::sync::{Arc, Mutex};

    // Arrange
    let fixture = FileSystemFixture::new()?;
    let base_dir = fixture.abs_path("concurrent_dir_test");
    create_dir_all(&base_dir)?;

    // Create 10 directories concurrently
    let errors = Arc::new(Mutex::new(Vec::new()));
    let mut handles = vec![];

    for i in 0..10 {
        let dir_path = format!("{}/dir_{}", base_dir.to_string_lossy(), i);
        let errors_clone = Arc::clone(&errors);

        let handle = thread::spawn(move || {
            // Create directory
            if let Err(e) = create_dir_all(&dir_path) {
                let mut errors = errors_clone.lock().unwrap();
                errors.push(format!("Create error: {}", e));
            }

            // Quick delay to increase chances of race conditions
            thread::sleep(std::time::Duration::from_millis(5));

            // Delete directory
            if let Err(e) = delete_dir_all(&dir_path) {
                let mut errors = errors_clone.lock().unwrap();
                errors.push(format!("Delete error: {}", e));
            }
        });

        handles.push(handle);
    }

    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }

    // Assert
    let errors = errors.lock().unwrap();
    assert!(errors.is_empty(), "Concurrent directory operations should not produce errors: {:?}", *errors);

    // Cleanup
    delete_dir_all(&base_dir)?;

    Ok(())
}