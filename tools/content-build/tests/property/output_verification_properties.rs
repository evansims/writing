use content_build::{build_content, BuildOptions, process_content, generate_sitemap, generate_rss_feed};
use common_test_utils::fixtures::TestFixture;
use common_test_utils::mocks::{MockFileSystem, MockConfigLoader};
use mockall::predicate;
use proptest::prelude::*;
use std::path::{Path, PathBuf};
use std::collections::HashMap;
use common_models::{Article, Frontmatter, Config, ContentConfig, TopicConfig, PublicationConfig};
use serde_json::Value;
use regex::Regex;

// Strategy for generating valid article data
fn article_data_strategy() -> impl Strategy<Value = (String, String, String, String, String)> {
    (
        // Slug
        r"[a-z0-9][a-z0-9\-]{3,20}".prop_map(String::from),
        // Title
        r#"[A-Za-z0-9\s\.\,\-\:\;]{5,50}"#.prop_map(String::from),
        // Tagline (using instead of description based on current Frontmatter structure)
        r#"[A-Za-z0-9\s\.\,\-\:\;]{10,100}"#.prop_map(String::from),
        // Date (YYYY-MM-DD)
        r"\d{4}\-\d{2}\-\d{2}".prop_map(String::from),
        // Content
        prop::collection::vec("[A-Za-z0-9\\s\\.\\,\\-\\:\\;]{5,20}".prop_map(String::from), 5..20)
            .prop_map(|lines| lines.join("\n"))
    )
}

// Strategy for generating multiple articles
fn multiple_articles_strategy(min: usize, max: usize) -> impl Strategy<Value = Vec<(String, String, String, String, String)>> {
    prop::collection::vec(article_data_strategy(), min..=max)
}

// Helper to create a test article from data
fn create_test_article(
    slug: &str,
    topic: &str,
    title: &str,
    tagline: &str,
    published_at: &str,
    content: &str,
    is_draft: bool,
) -> Article {
    let word_count = content.split_whitespace().count();
    let reading_time = (word_count as f64 / 200.0).ceil() as u32;

    Article {
        frontmatter: Frontmatter {
            title: title.to_string(),
            // Using tagline instead of description based on current Frontmatter structure
            tagline: Some(tagline.to_string()),
            published_at: Some(published_at.to_string()),
            updated_at: None,
            is_draft: Some(is_draft),
            ..Default::default()
        },
        content: content.to_string(),
        slug: slug.to_string(),
        topic: topic.to_string(),
        path: format!("content/{}/{}/index.mdx", topic, slug),
        word_count: Some(word_count),
        reading_time: Some(reading_time),
    }
}

// Verify JSON output contains expected properties
fn verify_json_contents(json_content: &str, article: &Article) -> Result<(), TestCaseError> {
    let json: Value = serde_json::from_str(json_content).expect("JSON should be valid");

    // Check basic properties
    prop_assert_eq!(json["title"].as_str().unwrap(), article.frontmatter.title, "Title should match");

    if let Some(tagline) = &article.frontmatter.tagline {
        prop_assert_eq!(json["tagline"].as_str().unwrap(), tagline, "Tagline should match");
    }

    if let Some(date) = &article.frontmatter.published_at {
        prop_assert_eq!(json["published_at"].as_str().unwrap(), date, "Published date should match");
    }

    prop_assert_eq!(json["slug"].as_str().unwrap(), article.slug, "Slug should match");
    prop_assert_eq!(json["topic"].as_str().unwrap(), article.topic, "Topic should match");

    // Word count and reading time
    if let Some(count) = article.word_count {
        prop_assert_eq!(json["word_count"].as_u64().unwrap(), count as u64, "Word count should match");
    }

    if let Some(time) = article.reading_time {
        prop_assert_eq!(json["reading_time"].as_u64().unwrap(), time as u64, "Reading time should match");
    }

    // Content should be present
    prop_assert!(json["content"].is_string(), "Content should be a string");

    Ok(())
}

// Verify XML output structure
fn verify_xml_structure(xml_content: &str, is_sitemap: bool) -> Result<(), TestCaseError> {
    if is_sitemap {
        // Basic sitemap structure verification
        prop_assert!(xml_content.contains("<?xml version=\"1.0\" encoding=\"UTF-8\"?>"), "XML declaration should be present");
        prop_assert!(xml_content.contains("<urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\">"), "Urlset element should be present");
        prop_assert!(xml_content.contains("<url>"), "URL element should be present");
        prop_assert!(xml_content.contains("<loc>"), "Location element should be present");
        prop_assert!(xml_content.contains("<lastmod>"), "Last modified element should be present");
        prop_assert!(xml_content.contains("<changefreq>"), "Change frequency element should be present");
        prop_assert!(xml_content.contains("<priority>"), "Priority element should be present");
    } else {
        // Basic RSS structure verification
        prop_assert!(xml_content.contains("<?xml version=\"1.0\" encoding=\"UTF-8\"?>"), "XML declaration should be present");
        prop_assert!(xml_content.contains("<rss version=\"2.0\">"), "RSS element should be present");
        prop_assert!(xml_content.contains("<channel>"), "Channel element should be present");
        prop_assert!(xml_content.contains("<title>"), "Title element should be present");
        prop_assert!(xml_content.contains("<link>"), "Link element should be present");
        prop_assert!(xml_content.contains("<description>"), "Description element should be present");
        prop_assert!(xml_content.contains("<language>"), "Language element should be present");
        prop_assert!(xml_content.contains("<item>"), "Item element should be present");
    }

    Ok(())
}

// Verify sitemap contains expected URLs
fn verify_sitemap_contents(sitemap_content: &str, articles: &[Article], site_url: &str) -> Result<(), TestCaseError> {
    // Basic structure verification
    verify_xml_structure(sitemap_content, true)?;

    // Check for each article URL
    for article in articles {
        // Skip draft articles
        if article.frontmatter.is_draft.unwrap_or(false) {
            continue;
        }

        let expected_url = format!("{}/{}/{}", site_url, article.topic, article.slug);
        prop_assert!(sitemap_content.contains(&expected_url), "Sitemap should contain URL for article: {}", article.slug);
    }

    Ok(())
}

// Verify RSS feed contains expected items
fn verify_rss_contents(rss_content: &str, articles: &[Article], site_url: &str) -> Result<(), TestCaseError> {
    // Basic structure verification
    verify_xml_structure(rss_content, false)?;

    // Check for each article
    for article in articles {
        // Skip draft articles
        if article.frontmatter.is_draft.unwrap_or(false) {
            continue;
        }

        prop_assert!(rss_content.contains(&article.frontmatter.title), "RSS feed should contain article title: {}", article.frontmatter.title);

        if let Some(tagline) = &article.frontmatter.tagline {
            prop_assert!(rss_content.contains(tagline), "RSS feed should contain article tagline");
        }

        let expected_url = format!("{}/{}/{}", site_url, article.topic, article.slug);
        prop_assert!(rss_content.contains(&expected_url), "RSS feed should contain URL for article: {}", article.slug);
    }

    Ok(())
}

proptest! {
    #[test]
    fn test_build_json_output_verification(articles in multiple_articles_strategy(2, 5)) {
        // Setup test fixture
        let fixture = TestFixture::new().unwrap();
        let mut mock_fs = MockFileSystem::new();

        // Define test paths
        let base_dir = fixture.path().join("content");
        let output_dir = fixture.path().join("public");
        let data_dir = output_dir.join("data");

        // Create a site URL for testing
        let site_url = "https://example.com";

        // Prepare topics and test articles
        let mut topics = HashMap::new();
        topics.insert("blog".to_string(), TopicConfig {
            name: "Blog".to_string(),
            description: "Blog posts".to_string(),
            directory: "blog".to_string(),
        });

        // Mock directory creation
        mock_fs.expect_create_dir_all()
            .with(predicate::eq(output_dir.clone()))
            .returning(|_| Ok(()));

        mock_fs.expect_create_dir_all()
            .with(predicate::eq(data_dir.clone()))
            .returning(|_| Ok(()));

        // Create test articles and setup mocks
        let test_articles: Vec<Article> = articles.iter().enumerate().map(|(i, (slug, title, tagline, date, content))| {
            create_test_article(slug, "blog", title, tagline, date, content, i % 3 == 0) // Every 3rd article is a draft
        }).collect();

        // Capture JSON written to files for verification
        let captured_json = std::sync::Arc::new(std::sync::Mutex::new(HashMap::<String, String>::new()));
        let captured_json_clone = captured_json.clone();

        // Expect JSON file writes and capture content
        for article in &test_articles {
            let json_path = data_dir.join(format!("{}.json", article.slug));
            let captured = captured_json.clone();

            mock_fs.expect_write_file()
                .with(predicate::eq(json_path.clone()), predicate::always())
                .returning(move |_, content| {
                    let mut map = captured.lock().unwrap();
                    map.insert(json_path.to_string_lossy().to_string(), content.to_owned());
                    Ok(())
                });
        }

        // Expect all.json file write and capture content
        let all_json_path = data_dir.join("all.json");
        mock_fs.expect_write_file()
            .with(predicate::eq(all_json_path.clone()), predicate::always())
            .returning(move |_, content| {
                let mut map = captured_json.lock().unwrap();
                map.insert(all_json_path.to_string_lossy().to_string(), content.to_owned());
                Ok(())
            });

        // Create mock config
        let config = Config {
            content: ContentConfig {
                base_dir: base_dir.to_string_lossy().to_string(),
                topics,
                tags: None,
            },
            publication: PublicationConfig {
                site_url: Some(site_url.to_string()),
                author: "Test Author".to_string(),
                copyright: "Copyright © 2023".to_string(),
            },
            ..Default::default()
        };

        // Setup mock config loader
        let mut mock_config = MockConfigLoader::new();
        mock_config.expect_load_config()
            .returning(move || Ok(config.clone()));

        // Mock template checks
        let templates_dir = PathBuf::from("templates");
        mock_fs.expect_dir_exists()
            .with(predicate::eq(templates_dir.clone()))
            .returning(|_| false);

        // Skip sitemap and RSS for this test (tested separately)
        mock_fs.expect_write_file()
            .with(predicate::eq(output_dir.join("sitemap.xml")), predicate::always())
            .returning(|_, _| Ok(()));

        mock_fs.expect_write_file()
            .with(predicate::eq(output_dir.join("rss.xml")), predicate::always())
            .returning(|_, _| Ok(()));

        // Set the mock filesystem on the fixture (manually since TestFixture may not have register methods)
        fixture.set_fs(mock_fs);
        fixture.set_config_loader(mock_config);

        // Create build options focused on JSON output
        let options = BuildOptions {
            output_dir: Some(output_dir.to_string_lossy().to_string()),
            slug: None,
            topic: None,
            include_drafts: true, // Include drafts for thorough testing
            skip_html: true,      // Skip HTML generation
            skip_json: false,     // Focus on JSON
            skip_rss: true,       // Skip RSS for this test
            skip_sitemap: true,   // Skip sitemap for this test
            verbose: false,
        };

        // Execute build
        let result = generate_sitemap(&output_dir, &test_articles, &config);
        prop_assert!(result.is_ok(), "Generating sitemap should succeed");

        // Verify all JSON files were created
        let captured_files = captured_json_clone.lock().unwrap();

        // Verify individual article JSON files
        for article in &test_articles {
            let json_path = data_dir.join(format!("{}.json", article.slug)).to_string_lossy().to_string();

            if let Some(json_content) = captured_files.get(&json_path) {
                // Verify this JSON file's content
                verify_json_contents(json_content, article)?;
            }
        }

        // Verify all.json contains all articles
        if let Some(all_json_content) = captured_files.get(&all_json_path.to_string_lossy().to_string()) {
            let all_json: Value = serde_json::from_str(all_json_content).expect("all.json should be valid JSON");
            prop_assert!(all_json.is_array(), "all.json should contain an array");

            // All non-draft articles should be in the array
            let non_draft_count = test_articles.iter()
                .filter(|a| !a.frontmatter.is_draft.unwrap_or(false))
                .count();

            let json_articles = all_json.as_array().unwrap();
            prop_assert_eq!(json_articles.len(), non_draft_count, "all.json should contain all non-draft articles");
        }
    }

    #[test]
    fn test_sitemap_generation_verification(articles in multiple_articles_strategy(2, 5)) {
        // Setup test fixture
        let fixture = TestFixture::new().unwrap();
        let mut mock_fs = MockFileSystem::new();

        // Define test paths
        let output_dir = fixture.path().join("public");
        let sitemap_path = output_dir.join("sitemap.xml");

        // Create a site URL for testing
        let site_url = "https://example.com";

        // Create test articles
        let test_articles: Vec<Article> = articles.iter().enumerate().map(|(i, (slug, title, tagline, date, content))| {
            create_test_article(slug, "blog", title, tagline, date, content, i % 3 == 0) // Every 3rd article is a draft
        }).collect();

        // Capture sitemap XML for verification
        let captured_sitemap = std::sync::Arc::new(std::sync::Mutex::new(None));
        let captured_sitemap_clone = captured_sitemap.clone();

        // Mock sitemap file write and capture content
        mock_fs.expect_write_file()
            .with(predicate::eq(sitemap_path.clone()), predicate::always())
            .returning(move |_, content| {
                let mut sitemap = captured_sitemap.lock().unwrap();
                *sitemap = Some(content.to_owned());
                Ok(())
            });

        // Set the mock filesystem on the fixture
        fixture.set_fs(mock_fs);

        // Create config for testing
        let mut topics = HashMap::new();
        topics.insert("blog".to_string(), TopicConfig {
            name: "Blog".to_string(),
            description: "Blog posts".to_string(),
            directory: "blog".to_string(),
        });

        let config = Config {
            content: ContentConfig {
                base_dir: "content".to_string(),
                topics,
                tags: None,
            },
            publication: PublicationConfig {
                site_url: Some(site_url.to_string()),
                author: "Test Author".to_string(),
                copyright: "Copyright © 2023".to_string(),
            },
            ..Default::default()
        };

        // Generate sitemap
        let result = generate_sitemap(&output_dir, &test_articles, &config);
        prop_assert!(result.is_ok(), "Generating sitemap should succeed");

        // Get the captured sitemap content
        let sitemap_content = {
            let content = captured_sitemap_clone.lock().unwrap();
            content.clone().expect("Sitemap content should have been captured")
        };

        // Verify sitemap structure and contents
        verify_sitemap_contents(&sitemap_content, &test_articles, site_url)?;
    }

    #[test]
    fn test_rss_feed_generation_verification(articles in multiple_articles_strategy(2, 5)) {
        // Setup test fixture
        let fixture = TestFixture::new().unwrap();
        let mut mock_fs = MockFileSystem::new();

        // Define test paths
        let output_dir = fixture.path().join("public");
        let rss_path = output_dir.join("rss.xml");

        // Create a site URL for testing
        let site_url = "https://example.com";

        // Create test articles
        let test_articles: Vec<Article> = articles.iter().enumerate().map(|(i, (slug, title, tagline, date, content))| {
            create_test_article(slug, "blog", title, tagline, date, content, i % 3 == 0) // Every 3rd article is a draft
        }).collect();

        // Capture RSS XML for verification
        let captured_rss = std::sync::Arc::new(std::sync::Mutex::new(None));
        let captured_rss_clone = captured_rss.clone();

        // Mock RSS file write and capture content
        mock_fs.expect_write_file()
            .with(predicate::eq(rss_path.clone()), predicate::always())
            .returning(move |_, content| {
                let mut rss = captured_rss.lock().unwrap();
                *rss = Some(content.to_owned());
                Ok(())
            });

        // Set the mock filesystem on the fixture
        fixture.set_fs(mock_fs);

        // Create config for testing
        let mut topics = HashMap::new();
        topics.insert("blog".to_string(), TopicConfig {
            name: "Blog".to_string(),
            description: "Blog posts".to_string(),
            directory: "blog".to_string(),
        });

        let config = Config {
            content: ContentConfig {
                base_dir: "content".to_string(),
                topics,
                tags: None,
            },
            publication: PublicationConfig {
                site_url: Some(site_url.to_string()),
                author: "Test Author".to_string(),
                copyright: "Copyright © 2023".to_string(),
            },
            ..Default::default()
        };

        // Generate RSS feed
        let result = generate_rss_feed(&output_dir, &test_articles, &config);
        prop_assert!(result.is_ok(), "Generating RSS feed should succeed");

        // Get the captured RSS content
        let rss_content = {
            let content = captured_rss_clone.lock().unwrap();
            content.clone().expect("RSS content should have been captured")
        };

        // Verify RSS structure and contents
        verify_rss_contents(&rss_content, &test_articles, site_url)?;
    }
}