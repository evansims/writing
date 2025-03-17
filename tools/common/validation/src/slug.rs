use common_errors::{Result, WritingError};
use regex::Regex;

/// Generate a slug from a title
///
/// This function takes a title string and converts it to a URL-friendly slug.
/// The slug will be lowercase, with spaces replaced by hyphens, and all non-alphanumeric
/// characters removed. The function also handles special Unicode characters by
/// first normalizing them.
///
/// # Arguments
///
/// * `title` - The title to convert to a slug
///
/// # Returns
///
/// A string containing the generated slug
///
/// # Examples
///
/// ```
/// use common_validation::slugify;
///
/// let title = "Hello World!";
/// let slug = slugify(title);
/// assert_eq!(slug, "hello-world");
/// ```
pub fn slugify(title: &str) -> String {
    // First, filter out any potential control characters or invalid Unicode
    let filtered_title = title.chars()
        .filter(|&c| !c.is_control() && c != '\u{FFFC}') // Filter out control chars and object replacement character
        .collect::<String>();
    
    // Use the slug crate for the actual slugification
    let raw_slug = slug::slugify(&filtered_title);
    
    // Ensure the slug is valid by checking for consecutive hyphens
    // and replacing them with a single hyphen
    let mut result = String::with_capacity(raw_slug.len());
    let mut last_was_hyphen = false;
    
    for c in raw_slug.chars() {
        if c == '-' {
            if !last_was_hyphen {
                result.push(c);
            }
            last_was_hyphen = true;
        } else {
            result.push(c);
            last_was_hyphen = false;
        }
    }
    
    // Ensure the slug is not too long
    const MAX_SLUG_LENGTH: usize = 100;
    if result.len() > MAX_SLUG_LENGTH {
        result.truncate(MAX_SLUG_LENGTH);
    }
    
    // Ensure the slug doesn't end with a hyphen
    if result.ends_with('-') {
        result.pop();
    }
    
    // If the slug is empty after all processing, return a default
    if result.is_empty() {
        return "untitled".to_string();
    }
    
    result
}

/// Validate that a slug is provided and properly formatted
///
/// This function checks that a slug meets the following criteria:
/// - Not empty
/// - Contains only lowercase letters, numbers, and hyphens
/// - Does not start or end with a hyphen
/// - Does not contain consecutive hyphens
///
/// # Arguments
///
/// * `slug` - The slug to validate
///
/// # Returns
///
/// * `Ok(String)` - The validated slug
/// * `Err(WritingError)` - An error if the slug is invalid
///
/// # Examples
///
/// ```
/// use common_validation::validate_slug;
///
/// let slug = "hello-world";
/// let result = validate_slug(slug);
/// assert!(result.is_ok());
///
/// let invalid_slug = "Hello World!";
/// let result = validate_slug(invalid_slug);
/// assert!(result.is_err());
/// ```
#[allow(dead_code)]
pub fn validate_slug(slug: &str) -> Result<String> {
    // Check if slug is empty
    if slug.is_empty() {
        return Err(WritingError::validation_error("Slug cannot be empty"));
    }
    
    // Check slug length - absolute maximum is 100 characters
    let max_length = 100;
    
    // Use byte length for comparison to be safe
    if slug.len() > max_length {
        return Err(WritingError::validation_error(
            format!("Slug is too long: {} bytes (maximum is {} bytes)", slug.len(), max_length)
        ));
    }
    
    // Check if slug contains only valid characters
    let slug_regex = Regex::new(r"^[a-z0-9-]+$").unwrap();
    if !slug_regex.is_match(slug) {
        return Err(WritingError::validation_error(
            "Slug can only contain lowercase letters, numbers, and hyphens"
        ));
    }
    
    // Check if slug starts or ends with a hyphen
    if slug.starts_with('-') || slug.ends_with('-') {
        return Err(WritingError::validation_error(
            "Slug cannot start or end with a hyphen"
        ));
    }
    
    // Check if slug contains consecutive hyphens
    if slug.contains("--") {
        return Err(WritingError::validation_error(
            "Slug cannot contain consecutive hyphens"
        ));
    }
    
    Ok(slug.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_validate_slug_valid() {
        let result = validate_slug("test-slug");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "test-slug");
    }
    
    #[test]
    fn test_validate_slug_empty() {
        let result = validate_slug("");
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("cannot be empty"));
    }
    
    #[test]
    fn test_validate_slug_invalid_chars() {
        let result = validate_slug("test_slug");
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("can only contain"));
    }
    
    #[test]
    fn test_validate_slug_starts_with_hyphen() {
        let result = validate_slug("-test-slug");
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("cannot start or end with a hyphen"));
    }
    
    #[test]
    fn test_validate_slug_ends_with_hyphen() {
        let result = validate_slug("test-slug-");
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("cannot start or end with a hyphen"));
    }
    
    #[test]
    fn test_validate_slug_consecutive_hyphens() {
        let result = validate_slug("test--slug");
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("cannot contain consecutive hyphens"));
    }
    
    #[test]
    fn test_generate_slug() {
        let title = "Test Title";
        let slug = slugify(title);
        assert_eq!(slug, "test-title");
    }
} 