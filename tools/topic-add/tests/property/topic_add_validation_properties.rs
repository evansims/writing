use proptest::prelude::*;
use proptest::prop_compose;
use topic_add::{add_topic, AddOptions};
use serial_test::serial;

// Generate invalid keys
prop_compose! {
    fn invalid_keys()(
        key in prop::string::string_regex("[a-z0-9-]{0,0}").unwrap()
    ) -> String {
        key
    }
}

// Generate valid keys
prop_compose! {
    fn valid_keys()(
        key in prop::string::string_regex("[a-z0-9-]{1,30}").unwrap()
    ) -> String {
        key
    }
}

// Generate valid names
prop_compose! {
    fn valid_names()(
        name in prop::string::string_regex("[A-Za-z0-9 !&()-]{1,30}").unwrap()
    ) -> String {
        name
    }
}

// Generate valid descriptions
prop_compose! {
    fn valid_descriptions()(
        desc in prop::string::string_regex("[A-Za-z0-9 !&(),.-]{5,50}").unwrap()
    ) -> String {
        desc
    }
}

// Generate valid directories
prop_compose! {
    fn valid_directories()(
        dir in prop::string::string_regex("[a-z0-9-]{1,30}").unwrap()
    ) -> String {
        dir
    }
}

// Property test: empty key validation
proptest! {
    #[test]
    #[serial]
    fn prop_empty_key_is_invalid(
        name in valid_names(),
        description in valid_descriptions(),
        directory in valid_directories()
    ) {
        // Create options with an empty key
        let options = AddOptions {
            key: String::new(),
            name,
            description,
            directory,
        };

        // Act
        let result = add_topic(&options);

        // Assert
        prop_assert!(result.is_err());
        prop_assert!(result.unwrap_err().to_string().contains("key is required"));
    }
}

// Property test: empty name validation
proptest! {
    #[test]
    #[serial]
    fn prop_empty_name_is_invalid(
        key in valid_keys(),
        description in valid_descriptions(),
        directory in valid_directories()
    ) {
        // Create options with an empty name
        let options = AddOptions {
            key,
            name: String::new(),
            description,
            directory,
        };

        // Act
        let result = add_topic(&options);

        // Assert
        prop_assert!(result.is_err());
        prop_assert!(result.unwrap_err().to_string().contains("name is required"));
    }
}

// Property test: empty description validation
proptest! {
    #[test]
    #[serial]
    fn prop_empty_description_is_invalid(
        key in valid_keys(),
        name in valid_names(),
        directory in valid_directories()
    ) {
        // Create options with an empty description
        let options = AddOptions {
            key,
            name,
            description: String::new(),
            directory,
        };

        // Act
        let result = add_topic(&options);

        // Assert
        prop_assert!(result.is_err());
        prop_assert!(result.unwrap_err().to_string().contains("description is required"));
    }
}

// Property test: empty directory validation
proptest! {
    #[test]
    #[serial]
    fn prop_empty_directory_is_invalid(
        key in valid_keys(),
        name in valid_names(),
        description in valid_descriptions()
    ) {
        // Create options with an empty directory
        let options = AddOptions {
            key,
            name,
            description,
            directory: String::new(),
        };

        // Act
        let result = add_topic(&options);

        // Assert
        prop_assert!(result.is_err());
        prop_assert!(result.unwrap_err().to_string().contains("directory is required"));
    }
}