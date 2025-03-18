use std::collections::HashMap;
use std::path::PathBuf;
use common_errors::Result;
use common_traits::tools::{ContentEditor, EditOptions};
use crate::impl_::find::find_content_path;
use crate::impl_::edit::{edit_content as edit_content_impl, update_frontmatter_field as update_field};
use crate::errors::ContentEditError;
use serde_json;

/// Implementation of the ContentEditor trait
pub struct ContentEditorImpl;

impl ContentEditorImpl {
    /// Create a new ContentEditorImpl
    pub fn new() -> Self {
        Self
    }
}

impl ContentEditor for ContentEditorImpl {
    fn edit_content(&self, options: &EditOptions) -> Result<PathBuf> {
        // Convert from common EditOptions to our internal options
        let internal_options = crate::models::EditOptions {
            slug: options.slug.clone(),
            topic: options.topic.clone(),
            frontmatter_only: options.field.is_some(), // Set this based on whether a field is specified
            content_only: false, // We don't have a direct mapping for this
        };

        // Call our internal implementation
        let content = edit_content_impl(&internal_options)
            .map_err(|e| common_errors::WritingError::from(e))?;

        // Return the path
        Ok(content.path)
    }

    fn update_frontmatter_field(&self, slug: &str, topic: Option<&str>, field: &str, value: &str) -> Result<()> {
        // Call our internal implementation
        update_field(slug, topic, field, value)
            .map_err(|e| common_errors::WritingError::from(e))
    }

    fn get_frontmatter_fields(&self, slug: &str, topic: Option<&str>) -> Result<HashMap<String, String>> {
        // Find the content
        let content_path = find_content_path(slug, topic)
            .map_err(|e| common_errors::WritingError::from(e))?;

        // Read the content
        let content = common_fs::read_file(&content_path)
            .map_err(|e| ContentEditError::FileSystem {
                error: std::io::Error::new(std::io::ErrorKind::Other, e.to_string())
            })
            .map_err(common_errors::WritingError::from)?;

        // Extract the frontmatter
        let yaml = crate::impl_::frontmatter::extract_frontmatter_from_string(&content)
            .map_err(|e| ContentEditError::InvalidFormat {
                reason: format!("Failed to extract frontmatter: {}", e)
            })
            .map_err(common_errors::WritingError::from)?;

        // Convert YAML to HashMap
        let mut fields = HashMap::new();

        for (key, value) in yaml.as_mapping().unwrap_or(&serde_yaml::Mapping::new()) {
            if let Some(key_str) = key.as_str() {
                match value {
                    serde_yaml::Value::String(s) => {
                        fields.insert(key_str.to_string(), s.clone());
                    },
                    serde_yaml::Value::Bool(b) => {
                        fields.insert(key_str.to_string(), b.to_string());
                    },
                    serde_yaml::Value::Number(n) => {
                        fields.insert(key_str.to_string(), n.to_string());
                    },
                    serde_yaml::Value::Null => {
                        fields.insert(key_str.to_string(), "null".to_string());
                    },
                    _ => {
                        // For complex types, convert to JSON string
                        if let Ok(json) = serde_json::to_string(value) {
                            fields.insert(key_str.to_string(), json);
                        }
                    }
                }
            }
        }

        Ok(fields)
    }
}