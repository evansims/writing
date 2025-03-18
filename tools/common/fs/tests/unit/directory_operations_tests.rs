use common_errors::Result;
use common_fs::{
    create_dir_all, delete_dir_all, path_exists, write_file,
    directory::has_content
};
use common_test_utils::FileSystemFixture;

#[test]
fn test_directory_operations_basic() -> Result<()> {
    // Create a temp directory for testing
    let fixture = FileSystemFixture::new()?;

    // Test directory path
    let test_dir = fixture.abs_path("test_dir");

    // Test create_dir_all
    create_dir_all(&test_dir)?;
    assert!(fixture.dir_exists("test_dir"));

    // Test dir_exists
    assert!(has_content(&test_dir) || path_exists(&test_dir));

    // Test delete_dir_all
    delete_dir_all(&test_dir)?;
    assert!(!fixture.dir_exists("test_dir"));

    Ok(())
}

#[test]
fn test_directory_operations_with_nested_paths() -> Result<()> {
    // Create a temp directory for testing
    let fixture = FileSystemFixture::new()?;

    // Test nested directory path
    let nested_dir = fixture.abs_path("parent/child/grandchild");

    // Test create_dir_all with nested directories
    create_dir_all(&nested_dir)?;
    assert!(fixture.dir_exists("parent"));
    assert!(fixture.dir_exists("parent/child"));
    assert!(fixture.dir_exists("parent/child/grandchild"));

    // Test delete_dir_all with nested directories
    delete_dir_all(&fixture.abs_path("parent"))?;
    assert!(!fixture.dir_exists("parent"));
    assert!(!fixture.dir_exists("parent/child"));
    assert!(!fixture.dir_exists("parent/child/grandchild"));

    Ok(())
}

#[cfg(feature = "directory_ops")]
#[test]
fn test_copy_dir_all() -> Result<()> {
    // Create a temp directory for testing
    let fixture = FileSystemFixture::new()?;

    // Create source directory with files
    let source_dir = fixture.abs_path("source_dir");
    create_dir_all(&source_dir)?;

    // Create files in source directory
    write_file(&source_dir.join("file1.txt"), "Content of file 1")?;
    write_file(&source_dir.join("file2.txt"), "Content of file 2")?;

    // Create nested directory with files
    let nested_dir = source_dir.join("nested");
    create_dir_all(&nested_dir)?;
    write_file(&nested_dir.join("nested_file.txt"), "Nested file content")?;

    // Target directory for copy
    let target_dir = fixture.abs_path("target_dir");

    // Test copy_dir_all
    copy_dir_all(&source_dir, &target_dir)?;

    // Verify target directory structure and content
    assert!(fixture.dir_exists("target_dir"));
    assert!(fixture.file_exists("target_dir/file1.txt"));
    assert!(fixture.file_exists("target_dir/file2.txt"));
    assert!(fixture.dir_exists("target_dir/nested"));
    assert!(fixture.file_exists("target_dir/nested/nested_file.txt"));

    // Verify file contents
    assert_eq!(
        fs::read_to_string(fixture.abs_path("target_dir/file1.txt"))?,
        "Content of file 1"
    );
    assert_eq!(
        fs::read_to_string(fixture.abs_path("target_dir/file2.txt"))?,
        "Content of file 2"
    );
    assert_eq!(
        fs::read_to_string(fixture.abs_path("target_dir/nested/nested_file.txt"))?,
        "Nested file content"
    );

    Ok(())
}

#[cfg(feature = "directory_ops")]
#[test]
fn test_move_dir() -> Result<()> {
    // Create a temp directory for testing
    let fixture = FileSystemFixture::new()?;

    // Create source directory with files
    let source_dir = fixture.abs_path("source_move_dir");
    create_dir_all(&source_dir)?;

    // Create files in source directory
    write_file(&source_dir.join("file1.txt"), "Content of file 1")?;
    write_file(&source_dir.join("file2.txt"), "Content of file 2")?;

    // Create nested directory with files
    let nested_dir = source_dir.join("nested");
    create_dir_all(&nested_dir)?;
    write_file(&nested_dir.join("nested_file.txt"), "Nested file content")?;

    // Target directory for move
    let target_dir = fixture.abs_path("target_move_dir");

    // Test move_dir
    move_dir(&source_dir, &target_dir)?;

    // Verify source directory no longer exists
    assert!(!fixture.dir_exists("source_move_dir"));

    // Verify target directory structure and content
    assert!(fixture.dir_exists("target_move_dir"));
    assert!(fixture.file_exists("target_move_dir/file1.txt"));
    assert!(fixture.file_exists("target_move_dir/file2.txt"));
    assert!(fixture.dir_exists("target_move_dir/nested"));
    assert!(fixture.file_exists("target_move_dir/nested/nested_file.txt"));

    // Verify file contents
    assert_eq!(
        fs::read_to_string(fixture.abs_path("target_move_dir/file1.txt"))?,
        "Content of file 1"
    );
    assert_eq!(
        fs::read_to_string(fixture.abs_path("target_move_dir/file2.txt"))?,
        "Content of file 2"
    );
    assert_eq!(
        fs::read_to_string(fixture.abs_path("target_move_dir/nested/nested_file.txt"))?,
        "Nested file content"
    );

    Ok(())
}

#[test]
fn test_dir_exists() {
    // Create a temp directory for testing
    let fixture = FileSystemFixture::new().unwrap();

    // Test dir path
    let test_dir = fixture.abs_path("test_dir_exists");

    // Before creating the directory
    assert!(!path_exists(&test_dir));

    // After creating the directory
    create_dir_all(&test_dir).unwrap();
    assert!(path_exists(&test_dir));

    // After deleting the directory
    delete_dir_all(&test_dir).unwrap();
    assert!(!path_exists(&test_dir));

    // With a file
    let file_path = fixture.abs_path("test_file.txt");
    write_file(&file_path, "test content").unwrap();
    assert!(path_exists(&file_path)); // Should return true for files
}