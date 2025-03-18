use common_errors::{Result, WritingError};
use common_models::Frontmatter;
use regex::Regex;

/// Validate content type
pub fn validate_content_type(content_type: &str) -> Result<String> {
    // Check if content type is empty
    if content_type.is_empty() {
        return Err(WritingError::validation_error("Content type cannot be empty"));
    }

    // Check if content type is valid
    let valid_types = ["article", "note", "tutorial"];
    if !valid_types.contains(&content_type) {
        return Err(WritingError::validation_error(format!(
            "Invalid content type: {}. Valid types are: {}",
            content_type,
            valid_types.join(", ")
        )));
    }

    Ok(content_type.to_string())
}

/// Validate content
pub fn validate_content(content: &str) -> Result<()> {
    // Check if content is empty
    if content.trim().is_empty() {
        return Err(WritingError::validation_error("Content cannot be empty"));
    }

    // Check if content has frontmatter
    if !content.starts_with("---") {
        return Err(WritingError::validation_error("Content must start with frontmatter"));
    }

    // Check if frontmatter is valid
    let parts: Vec<&str> = content.splitn(3, "---").collect();
    if parts.len() < 3 {
        return Err(WritingError::validation_error("Invalid frontmatter format"));
    }

    // Validate frontmatter
    let frontmatter = parts[1].trim();
    let parsed: Frontmatter = serde_yaml::from_str(frontmatter)
        .map_err(|e| WritingError::validation_error(format!("Invalid frontmatter: {}", e)))?;

    // Validate required fields
    if parsed.title.is_empty() {
        return Err(WritingError::validation_error("Title is required in frontmatter"));
    }

    Ok(())
}

/// Validate content body
pub fn validate_content_body(body: &str) -> Result<()> {
    // Check if body is empty
    if body.trim().is_empty() {
        return Err(WritingError::validation_error("Content body cannot be empty"));
    }

    Ok(())
}

/// Validate content title
pub fn validate_content_title(title: &str) -> Result<String> {
    // Check if title is empty
    if title.trim().is_empty() {
        return Err(WritingError::validation_error("Title cannot be empty"));
    }

    Ok(title.trim().to_string())
}

/// Validate content tagline
pub fn validate_content_tagline(tagline: &str) -> Result<String> {
    // Tagline is optional, so empty is fine
    Ok(tagline.trim().to_string())
}

/// Validate content date
pub fn validate_content_date(date: &str) -> Result<String> {
    // Check if date is empty
    if date.trim().is_empty() {
        return Err(WritingError::validation_error("Date cannot be empty"));
    }

    // Check if date has valid format (YYYY-MM-DD)
    let date_regex = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
    if !date_regex.is_match(date.trim()) {
        return Err(WritingError::validation_error(
            "Date must be in YYYY-MM-DD format"
        ));
    }

    Ok(date.trim().to_string())
}

/// Generate a template for a new content
pub fn generate_content_template(title: &str, tagline: Option<&str>, content_type: &str, tags: &[String]) -> Result<String> {
    // Validate content type
    validate_content_type(content_type)?;

    // Format tags
    let tags_yaml = if tags.is_empty() {
        "[]".to_string()
    } else {
        let tags_str = tags
            .iter()
            .map(|t| format!("  - \"{}\"", t))
            .collect::<Vec<_>>()
            .join("\n");

        format!("[\n{}\n]", tags_str)
    };

    // Generate frontmatter
    let frontmatter = format!(
        "title: \"{}\"\n\
         type: \"{}\"\n\
         date: \"{}\"\n\
         tags: {}\n\
         published: false",
        title,
        content_type,
        chrono::Local::now().format("%Y-%m-%d"),
        tags_yaml
    );

    // Add tagline if provided
    let frontmatter = if let Some(tagline) = tagline {
        format!("{}\ntagline: \"{}\"", frontmatter, tagline)
    } else {
        frontmatter
    };

    // Generate content template
    let template = format!(
        "---\n\
         {}\n\
         ---\n\n\
         # {}\n\n\
         Write your content here...",
        frontmatter,
        title
    );

    Ok(template)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_content_type_valid() {
        let result = validate_content_type("article");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "article");
    }

    #[test]
    fn test_validate_content_type_invalid() {
        let result = validate_content_type("invalid-type");
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("Invalid content type"));
    }

    #[test]
    fn test_validate_content_valid() {
        let content = "---\ntitle: \"Test Post\"\ntagline: \"A test post\"\n---\n\nThis is the body.";
        let result = validate_content(content);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_content_invalid_frontmatter() {
        let content = "---\ntagline: \"A test post\"\n---\n\nThis is the body.";
        let result = validate_content(content);
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("validation") || err.contains("required") || err.contains("missing"));
    }

    #[test]
    fn test_validate_content_body() {
        let body = "This is the body.";
        let result = validate_content_body(body);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_content_body_empty() {
        let body = "";
        let result = validate_content_body(body);
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("cannot be empty"));
    }

    #[test]
    fn test_validate_content_title() {
        let title = "Test Title";
        let result = validate_content_title(title);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Test Title");
    }

    #[test]
    fn test_validate_content_title_empty() {
        let title = "";
        let result = validate_content_title(title);
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("cannot be empty"));
    }

    #[test]
    fn test_validate_content_date() {
        let date = "2023-01-01";
        let result = validate_content_date(date);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "2023-01-01");
    }

    #[test]
    fn test_validate_content_date_invalid() {
        let date = "01/01/2023";
        let result = validate_content_date(date);
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("must be in YYYY-MM-DD format"));
    }

    #[test]
    fn test_generate_content_template() {
        let title = "Test Title";
        let tagline = Some("A test post");
        let content_type = "article";
        let tags = vec!["tag1".to_string(), "tag2".to_string()];

        let result = generate_content_template(title, tagline, content_type, &tags);
        assert!(result.is_ok());

        let template = result.unwrap();
        assert!(template.contains("title: \"Test Title\""));
        assert!(template.contains("tagline: \"A test post\""));
        assert!(template.contains("type: \"article\""));
        assert!(template.contains("  - \"tag1\""));
        assert!(template.contains("  - \"tag2\""));
        assert!(template.contains("# Test Title"));
    }
}