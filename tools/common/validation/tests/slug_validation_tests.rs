use common_test_utils::proptest::*;
use common_test_utils::ValidationFixture;
use common_validation::validate_slug;
use proptest::prelude::*;

#[test]
fn test_valid_slugs_with_fixture() {
    let fixture = ValidationFixture::new().unwrap();
    
    // Test all valid examples
    for slug in fixture.get_valid_examples("slug") {
        let result = validate_slug(&slug);
        assert!(result.is_ok(), "Valid slug '{}' was rejected: {:?}", slug, result.err());
        assert_eq!(result.unwrap(), slug);
    }
}

#[test]
fn test_invalid_slugs_with_fixture() {
    let fixture = ValidationFixture::new().unwrap();
    
    // Test all invalid examples
    for slug in fixture.get_invalid_examples("slug") {
        let result = validate_slug(&slug);
        assert!(result.is_err(), "Invalid slug '{}' was accepted", slug);
    }
}

proptest! {
    #[test]
    fn test_valid_slugs_property_based(slug in valid_slug_strategy()) {
        // All generated valid slugs should pass validation
        prop_assert!(validate_slug(&slug).is_ok());
    }
    
    #[test]
    fn test_invalid_slugs_property_based(slug in invalid_slug_strategy()) {
        // All generated invalid slugs should fail validation
        let result = validate_slug(&slug);
        prop_assert!(result.is_err());
    }
    
    #[test]
    fn test_slugify_produces_valid_slugs(title in "\\PC{1,100}") {
        // Any title should produce a valid slug
        let slug = common_validation::slugify(&title);
        prop_assert!(validate_slug(&slug).is_ok());
    }
} 