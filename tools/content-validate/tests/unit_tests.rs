#[cfg(test)]
mod tests {
    use content_validate::{
        extract_links,
        LocalLinkKind,
        ValidationOptions,
        ValidationType,
    };

    #[test]
    fn test_simple_assertion() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_extract_links_with_no_links() {
        let content = "This is a plain text with no links.";
        let links = extract_links(content);
        assert!(links.is_empty());
    }

    #[test]
    fn test_extract_links_with_markdown_links() {
        let content = "Check out [external link](https://example.com) and [internal link](/docs/example) and [another link](https://test.org)";

        let links = extract_links(content);
        assert_eq!(links.len(), 3);

        assert_eq!(links[0].url(), "https://example.com");
        assert_eq!(*links[0].kind(), LocalLinkKind::External);

        assert_eq!(links[1].url(), "/docs/example");
        assert_eq!(*links[1].kind(), LocalLinkKind::Internal);

        assert_eq!(links[2].url(), "https://test.org");
        assert_eq!(*links[2].kind(), LocalLinkKind::External);
    }

    #[test]
    fn test_validation_options_creation() {
        let options = ValidationOptions {
            article_slug: None,
            topic: None,
            validation_types: vec![],
            check_external_links: false,
            timeout: None,
            dictionary_path: None,
            include_drafts: false,
        };

        assert_eq!(options.article_slug, None);
        assert_eq!(options.validation_types.len(), 0);
        assert_eq!(options.dictionary_path, None);
    }

    #[test]
    fn test_validation_types() {
        let mut options = ValidationOptions {
            article_slug: None,
            topic: None,
            validation_types: vec![],
            check_external_links: false,
            timeout: None,
            dictionary_path: None,
            include_drafts: false,
        };

        options.validation_types.push(ValidationType::Links);
        options.validation_types.push(ValidationType::Markdown);

        assert_eq!(options.validation_types.len(), 2);
        assert_eq!(options.validation_types[0], ValidationType::Links);
        assert_eq!(options.validation_types[1], ValidationType::Markdown);
    }
}