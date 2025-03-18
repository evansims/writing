use common_errors::Result;
use common_fs::{read_file, write_file, delete_file, path_exists};
use common_test_utils::FileSystemFixture;
use proptest::prelude::*;
use std::path::PathBuf;

/// Strategy to generate valid file paths
fn valid_filepath_strategy() -> impl Strategy<Value = String> {
    // Generate valid path components
    let component_strategy = proptest::string::string_regex("[a-zA-Z0-9_\\-\\.]{1,10}").unwrap();

    // Generate 1-3 path components
    proptest::collection::vec(component_strategy, 1..4)
        .prop_map(|components| components.join("/"))
}

/// Strategy to generate file content of various sizes and types
fn file_content_strategy() -> impl Strategy<Value = String> {
    prop_oneof![
        // Empty content
        Just("".to_string()),

        // Small content (1-100 chars)
        proptest::string::string_regex(".{1,100}").unwrap(),

        // Medium content (101-1000 chars)
        proptest::string::string_regex(".{101,1000}").unwrap(),

        // Large content (1001-10000 chars)
        prop::collection::vec(proptest::string::string_regex(".{1,100}").unwrap(), 10..100)
            .prop_map(|v| v.join(""))
    ]
}

/// Strategy to generate valid unicode content
fn unicode_content_strategy() -> impl Strategy<Value = String> {
    prop_oneof![
        // ASCII content
        proptest::string::string_regex("[a-zA-Z0-9 ]{1,100}").unwrap(),

        // Mixed ASCII and unicode
        proptest::collection::vec(prop_oneof![
            Just("Hello, "),
            Just("こんにちは "),
            Just("你好 "),
            Just("안녕하세요 "),
            Just("Здравствуйте ")
        ], 1..10).prop_map(|v| v.join("")),

        // Emojis
        proptest::collection::vec(prop_oneof![
            Just("😀 "),
            Just("🚀 "),
            Just("💡 "),
            Just("🌍 "),
            Just("📚 ")
        ], 1..10).prop_map(|v| v.join(""))
    ]
}

proptest! {
    /// Property: A file written with write_file can be read back with read_file
    #[test]
    fn prop_write_and_read_file_content_preserved(
        relative_path in valid_filepath_strategy(),
        content in file_content_strategy()
    ) {
        // Arrange
        let fixture = FileSystemFixture::new().unwrap();
        let abs_path = fixture.abs_path(&relative_path);

        // Make sure parent directory exists
        if let Some(parent) = abs_path.parent() {
            std::fs::create_dir_all(parent).unwrap();
        }

        // Act
        write_file(&abs_path, &content).unwrap();
        let read_content = read_file(&abs_path).unwrap();

        // Assert
        prop_assert_eq!(content, read_content);
    }

    /// Property: Unicode content is preserved when written and read
    #[test]
    fn prop_unicode_content_preserved(
        relative_path in valid_filepath_strategy(),
        content in unicode_content_strategy()
    ) {
        // Arrange
        let fixture = FileSystemFixture::new().unwrap();
        let abs_path = fixture.abs_path(&relative_path);

        // Make sure parent directory exists
        if let Some(parent) = abs_path.parent() {
            std::fs::create_dir_all(parent).unwrap();
        }

        // Act
        write_file(&abs_path, &content).unwrap();
        let read_content = read_file(&abs_path).unwrap();

        // Assert
        prop_assert_eq!(content, read_content);
    }

    /// Property: After deleting a file, it should not exist
    #[test]
    fn prop_delete_file_removes_file(
        relative_path in valid_filepath_strategy(),
        content in file_content_strategy()
    ) {
        // Arrange
        let fixture = FileSystemFixture::new().unwrap();
        let abs_path = fixture.abs_path(&relative_path);

        // Make sure parent directory exists
        if let Some(parent) = abs_path.parent() {
            std::fs::create_dir_all(parent).unwrap();
        }

        // Create file
        write_file(&abs_path, &content).unwrap();
        prop_assert!(path_exists(&abs_path));

        // Act
        delete_file(&abs_path).unwrap();

        // Assert
        prop_assert!(!path_exists(&abs_path));
    }

    /// Property: Reading a non-existent file returns an error
    #[test]
    fn prop_read_nonexistent_file_returns_error(
        relative_path in valid_filepath_strategy()
    ) {
        // Arrange
        let fixture = FileSystemFixture::new().unwrap();
        let abs_path = fixture.abs_path(&relative_path);

        // Act
        let result = read_file(&abs_path);

        // Assert
        prop_assert!(result.is_err());
        let err = result.unwrap_err();
        prop_assert!(
            err.to_string().contains("not found") ||
            err.to_string().contains("No such file")
        );
    }

    /// Property: Deleting a non-existent file should not error
    #[test]
    fn prop_delete_nonexistent_file_does_not_error(
        relative_path in valid_filepath_strategy()
    ) {
        // Arrange
        let fixture = FileSystemFixture::new().unwrap();
        let abs_path = fixture.abs_path(&relative_path);

        // Act
        let result = delete_file(&abs_path);

        // Assert - delete_file should be idempotent and not error
        prop_assert!(result.is_ok());
    }
}