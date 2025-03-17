use common_errors::{Result, WritingError};
use common_models::Frontmatter;

/// Validate frontmatter
#[allow(dead_code)]
pub fn validate_frontmatter(frontmatter: &str) -> Result<Frontmatter> {
    // Check if frontmatter is empty
    if frontmatter.trim().is_empty() {
        return Err(WritingError::validation_error("Frontmatter cannot be empty"));
    }
    
    // Check if frontmatter has valid YAML format
    let frontmatter = frontmatter.trim();
    let frontmatter = if frontmatter.starts_with("---") {
        let parts: Vec<&str> = frontmatter.splitn(3, "---").collect();
        if parts.len() < 3 {
            return Err(WritingError::validation_error("Invalid frontmatter format"));
        }
        parts[1].trim()
    } else {
        frontmatter
    };
    
    // Parse frontmatter
    let parsed: Frontmatter = serde_yaml::from_str(frontmatter)
        .map_err(|e| WritingError::validation_error(format!("Invalid frontmatter: {}", e)))?;
    
    // Validate required fields
    if parsed.title.is_empty() {
        return Err(WritingError::validation_error("Title is required in frontmatter"));
    }
    
    Ok(parsed)
}

/// Extract frontmatter from content
pub fn extract_frontmatter(content: &str) -> Result<(String, String)> {
    if !content.starts_with("---") {
        return Err(WritingError::validation_error("Content does not contain frontmatter"));
    }
    
    let parts: Vec<&str> = content.splitn(3, "---").collect();
    if parts.len() < 3 {
        return Err(WritingError::validation_error("Invalid frontmatter format"));
    }
    
    let frontmatter = parts[1].trim();
    let body = parts[2].trim();
    
    Ok((frontmatter.to_string(), body.to_string()))
}

/// Combine frontmatter and body into content
pub fn combine_frontmatter_and_body(frontmatter: &str, body: &str) -> String {
    format!("---\n{}\n---\n\n{}", frontmatter.trim(), body.trim())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_validate_frontmatter_valid() {
        let frontmatter = "---\ntitle: \"Test Post\"\ntagline: \"A test post\"\n---";
        let result = validate_frontmatter(frontmatter);
        assert!(result.is_ok());
        let parsed = result.unwrap();
        assert_eq!(parsed.title, "Test Post");
        assert_eq!(parsed.tagline, Some("A test post".to_string()));
    }
    
    #[test]
    fn test_validate_frontmatter_invalid_format() {
        let frontmatter = "---\ntitle: \"Test Post\"\ntagline: \"A test post\"";
        let result = validate_frontmatter(frontmatter);
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("Invalid frontmatter format"));
    }
    
    #[test]
    fn test_validate_frontmatter_missing_title() {
        let frontmatter = "---\ntagline: \"A test post\"\n---";
        let result = validate_frontmatter(frontmatter);
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("validation") || err.contains("required") || err.contains("missing"));
    }
    
    #[test]
    fn test_extract_frontmatter() {
        let content = "---\ntitle: \"Test Post\"\ntagline: \"A test post\"\n---\n\nThis is the body.";
        let result = extract_frontmatter(content);
        assert!(result.is_ok());
        let (frontmatter, body) = result.unwrap();
        assert_eq!(frontmatter, "title: \"Test Post\"\ntagline: \"A test post\"");
        assert_eq!(body, "This is the body.");
    }
    
    #[test]
    fn test_extract_frontmatter_invalid() {
        let content = "This is the body.";
        let result = extract_frontmatter(content);
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("does not contain frontmatter"));
    }
    
    #[test]
    fn test_combine_frontmatter_and_body() {
        let frontmatter = "title: \"Test Post\"\ntagline: \"A test post\"";
        let body = "This is the body.";
        let content = combine_frontmatter_and_body(frontmatter, body);
        assert_eq!(content, "---\ntitle: \"Test Post\"\ntagline: \"A test post\"\n---\n\nThis is the body.");
    }
} 