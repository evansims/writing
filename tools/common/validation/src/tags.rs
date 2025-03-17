use common_errors::{Result, WritingError};
use regex::Regex;

/// Validate that tags are properly formatted
///
/// This function checks that tags meet the following criteria:
/// - Each tag contains only letters, numbers, spaces, hyphens, and underscores
/// - Tags are split by commas
///
/// # Arguments
///
/// * `tags` - A comma-separated string of tags to validate
///
/// # Returns
///
/// * `Ok(Vec<String>)` - A vector of validated tags
/// * `Err(WritingError)` - An error if any tag is invalid
///
/// # Examples
///
/// ```
/// use common_validation::validate_tags;
///
/// let tags = "rust, programming, web-development";
/// let result = validate_tags(tags);
/// assert!(result.is_ok());
///
/// let invalid_tags = "rust, programming!, web-development";
/// let result = validate_tags(invalid_tags);
/// assert!(result.is_err());
/// ```
#[allow(dead_code)]
pub fn validate_tags(tags: &str) -> Result<Vec<String>> {
    // Split tags by comma
    let tags: Vec<String> = tags
        .split(',')
        .map(|t| t.trim())
        .filter(|t| !t.is_empty())
        .map(|t| t.to_string())
        .collect();
    
    // Check if any tag contains invalid characters
    let tag_regex = Regex::new(r"^[a-zA-Z0-9-_ ]+$").unwrap();
    for tag in &tags {
        if !tag_regex.is_match(tag) {
            return Err(WritingError::validation_error(format!(
                "Tag '{}' contains invalid characters. Tags can only contain letters, numbers, spaces, hyphens, and underscores",
                tag
            )));
        }
    }
    
    Ok(tags)
}

/// Format tags for frontmatter
///
/// This function takes a comma-separated string of tags and formats them
/// for use in frontmatter. It trims whitespace, removes empty tags,
/// and returns a comma-separated string.
///
/// # Arguments
///
/// * `tags` - A comma-separated string of tags to format
///
/// # Returns
///
/// A formatted string of tags
///
/// # Examples
///
/// ```
/// use common_validation::format_tags;
///
/// let tags = "rust, programming, web-development";
/// let formatted = format_tags(tags);
/// assert_eq!(formatted, "    \"rust\",\n    \"programming\",\n    \"web-development\",");
///
/// let messy_tags = " rust,  programming ,web-development, ";
/// let formatted = format_tags(messy_tags);
/// assert_eq!(formatted, "    \"rust\",\n    \"programming\",\n    \"web-development\",");
/// ```
#[allow(dead_code)]
pub fn format_tags(tags: &str) -> String {
    if tags.is_empty() {
        return "".to_string();
    }
    
    tags.split(',')
        .map(|t| t.trim())
        .filter(|t| !t.is_empty())
        .map(|t| format!("    \"{}\",", t))
        .collect::<Vec<_>>()
        .join("\n")
}

/// Format tags as a YAML array
pub fn format_tags_yaml(tags: &[String]) -> String {
    if tags.is_empty() {
        return "[]".to_string();
    }
    
    let tags_str = tags
        .iter()
        .map(|t| format!("  - \"{}\"", t))
        .collect::<Vec<_>>()
        .join("\n");
    
    format!("[\n{}\n]", tags_str)
}

/// Parse tags from a YAML array
pub fn parse_tags_yaml(yaml: &str) -> Result<Vec<String>> {
    let yaml = yaml.trim();
    
    // Handle empty array
    if yaml == "[]" {
        return Ok(Vec::new());
    }
    
    // Handle inline array
    if yaml.starts_with('[') && yaml.ends_with(']') {
        let inner = &yaml[1..yaml.len() - 1];
        let tags = inner
            .split(',')
            .map(|t| t.trim())
            .filter(|t| !t.is_empty())
            .map(|t| {
                // Remove quotes if present
                let t = t.trim();
                if (t.starts_with('"') && t.ends_with('"')) || (t.starts_with('\'') && t.ends_with('\'')) {
                    t[1..t.len() - 1].to_string()
                } else {
                    t.to_string()
                }
            })
            .collect();
        
        return Ok(tags);
    }
    
    // Handle multiline array
    let tags = yaml
        .lines()
        .map(|line| line.trim())
        .filter(|line| line.starts_with('-'))
        .map(|line| {
            let tag = line[1..].trim();
            // Remove quotes if present
            if (tag.starts_with('"') && tag.ends_with('"')) || (tag.starts_with('\'') && tag.ends_with('\'')) {
                tag[1..tag.len() - 1].to_string()
            } else {
                tag.to_string()
            }
        })
        .collect();
    
    Ok(tags)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_validate_tags_valid() {
        let tags = "tag1, tag2, tag3";
        let result = validate_tags(tags);
        assert!(result.is_ok());
        let parsed = result.unwrap();
        assert_eq!(parsed, vec!["tag1", "tag2", "tag3"]);
    }
    
    #[test]
    fn test_validate_tags_invalid() {
        let tags = "tag1, tag@2, tag3";
        let result = validate_tags(tags);
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("contains invalid characters"));
    }
    
    #[test]
    fn test_format_tags() {
        let tags = "tag1, tag2, tag3";
        let formatted = format_tags(tags);
        assert_eq!(formatted, "    \"tag1\",\n    \"tag2\",\n    \"tag3\",");
    }
    
    #[test]
    fn test_format_tags_yaml() {
        let tags = vec!["tag1".to_string(), "tag2".to_string(), "tag3".to_string()];
        let formatted = format_tags_yaml(&tags);
        assert_eq!(formatted, "[\n  - \"tag1\"\n  - \"tag2\"\n  - \"tag3\"\n]");
    }
    
    #[test]
    fn test_parse_tags_yaml_empty() {
        let yaml = "[]";
        let result = parse_tags_yaml(yaml);
        assert!(result.is_ok());
        let parsed = result.unwrap();
        assert_eq!(parsed, Vec::<String>::new());
    }
    
    #[test]
    fn test_parse_tags_yaml_inline() {
        let yaml = "[\"tag1\", \"tag2\", \"tag3\"]";
        let result = parse_tags_yaml(yaml);
        assert!(result.is_ok());
        let parsed = result.unwrap();
        assert_eq!(parsed, vec!["tag1", "tag2", "tag3"]);
    }
    
    #[test]
    fn test_parse_tags_yaml_multiline() {
        let yaml = "- \"tag1\"\n- \"tag2\"\n- \"tag3\"";
        let result = parse_tags_yaml(yaml);
        assert!(result.is_ok());
        let parsed = result.unwrap();
        assert_eq!(parsed, vec!["tag1", "tag2", "tag3"]);
    }
} 