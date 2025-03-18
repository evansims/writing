//! Unit tests extracted from lib.rs

use content_move::*;

mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::TempDir;
    use std::fs;
    use std::path::Path;

    #[test]
    fn test_update_content_references() {
        // Create a temporary directory
        let temp_dir = TempDir::new().unwrap();
        let test_file_path = temp_dir.path().join("test_file.md");

        // Create test content
        let content = r#"
This is a test file that links to [old-slug](../old-slug/) and references old-slug multiple times.
Let's see if old-slug gets replaced correctly.
"#;

        // Write test content to file
        let mut file = fs::File::create(&test_file_path).unwrap();
        file.write_all(content.as_bytes()).unwrap();

        // Update references
        update_content_references(&test_file_path, "old-slug", "new-slug").unwrap();

        // Read updated content
        let updated_content = fs::read_to_string(&test_file_path).unwrap();

        // Check if references were updated
        assert!(!updated_content.contains("old-slug"));
        assert!(updated_content.contains("new-slug"));
    }

    #[test]
    fn test_imports_are_working() {
        assert!(true);
    }
}
