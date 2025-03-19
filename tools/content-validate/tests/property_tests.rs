#[cfg(test)]
mod property_tests {
    use content_validate::extract_links;

    #[test]
    fn test_extract_links_with_markdown_link() {
        let content = "[a link](https://example.com)";
        let links = extract_links(content);

        assert_eq!(links.len(), 1, "Expected exactly one link to be extracted");
        assert_eq!(links[0].url(), "https://example.com");
    }
}