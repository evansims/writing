use anyhow::Result;
use chrono::NaiveDate;
use common_models::{Config, Frontmatter, TopicConfig};
use comrak::{markdown_to_html, ComrakOptions};
use regex::Regex;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Structure to hold content statistics for an article
#[derive(Clone, Debug)]
pub struct ContentStats {
    pub title: String,
    pub published: String,
    pub word_count: usize,
    pub reading_time: usize, // in minutes
    pub character_count: usize,
    pub paragraph_count: usize,
    pub sentence_count: usize,
    pub topic: String,
    pub slug: String,
    pub tags: Vec<String>,
    pub is_draft: bool,
    // Add the fields needed for the overall stats
    pub total_articles: usize,
    pub total_words: usize,
    pub total_drafts: usize,
    pub total_published: usize,
    pub topics: Vec<TopicStats>,
}

/// Structure to hold statistics for a topic
#[derive(Clone, Debug)]
pub struct TopicStats {
    pub key: String,
    pub name: String,
    pub article_count: usize,
    pub word_count: usize,
    pub draft_count: usize,
    pub published_count: usize,
}

/// Options for content statistics generation
pub struct StatsOptions {
    pub slug: Option<String>,
    pub topic: Option<String>,
    pub include_drafts: bool,
    pub sort_by: String,
    pub detailed: bool,
}

/// Type alias for stats generation result
type StatsResult = (
    Vec<ContentStats>,
    HashMap<String, usize>,
    usize,
    usize,
    usize,
);

/// Calculate statistics for a single content file
pub fn calculate_stats(
    content: &str,
    frontmatter: &Frontmatter,
    topic: &str,
    slug: &str,
) -> ContentStats {
    // Strip HTML tags for accurate word count
    let options = ComrakOptions::default();
    let html = markdown_to_html(content, &options);

    // Use regex to remove HTML tags
    let re = Regex::new(r"<[^>]*>").unwrap();
    let text = re.replace_all(&html, "").to_string();

    let word_count = common_markdown::calculate_word_count(&text);
    let character_count = text.chars().count();

    // Count paragraphs (non-empty lines)
    let paragraph_count = content
        .lines()
        .filter(|line| !line.trim().is_empty())
        .count();

    // Count sentences (roughly)
    let sentence_re = Regex::new(r"[.!?]+").unwrap();
    let sentence_count = sentence_re.find_iter(&text).count();

    // Calculate reading time
    let reading_time = common_markdown::calculate_reading_time(word_count) as usize;

    // Extract tags
    let tags = frontmatter.tags.clone().unwrap_or_default();

    // Check if draft
    let is_draft = frontmatter.is_draft.unwrap_or(false)
        || frontmatter
            .published_at
            .as_ref()
            .is_some_and(|p| p == "DRAFT");

    // Get published date or default
    let published = frontmatter
        .published_at
        .clone()
        .unwrap_or_else(|| "DRAFT".to_string());

    ContentStats {
        title: frontmatter.title.clone(),
        published,
        word_count,
        reading_time,
        character_count,
        paragraph_count,
        sentence_count,
        topic: topic.to_string(),
        slug: slug.to_string(),
        tags,
        is_draft,
        total_articles: 0,
        total_words: 0,
        total_drafts: 0,
        total_published: 0,
        topics: Vec::new(),
    }
}

/// Format a date string for display
pub fn format_date(date_str: &str) -> String {
    if date_str == "DRAFT" {
        return "DRAFT".to_string();
    }

    if let Ok(date) = NaiveDate::parse_from_str(date_str, "%Y-%m-%d") {
        date.format("%b %d, %Y").to_string()
    } else {
        date_str.to_string()
    }
}

/// Generate statistics for content files based on the provided options
pub fn generate_stats(options: &StatsOptions) -> Result<StatsResult> {
    // Read configuration
    let config = common_config::load_config()?;

    // Validate topic if provided
    if let Some(ref topic) = options.topic {
        if !config.content.topics.contains_key(topic) {
            let valid_topics: Vec<String> = config
                .content
                .topics
                .keys()
                .map(|k| k.to_string())
                .collect();
            return Err(anyhow::anyhow!(
                "Invalid topic: {}. Valid topics are: {}",
                topic,
                valid_topics.join(", ")
            ));
        }
    }

    let mut all_stats: Vec<ContentStats> = Vec::new();
    let mut total_words = 0;
    let mut total_articles = 0;
    let mut total_drafts = 0;
    let mut tag_counts: HashMap<String, usize> = HashMap::new();

    // Get the content base directory
    let content_base_dir = PathBuf::from(&config.content.base_dir);

    // Process content for specific slug if provided
    if let Some(ref slug) = options.slug {
        // Find the article with the given slug in any topic
        let mut found = false;

        for (topic_key, topic_config) in &config.content.topics {
            // Skip if a specific topic is requested and it's not this one
            if options.topic.is_some() && options.topic.as_ref() != Some(topic_key) {
                continue;
            }

            let topic_path = &topic_config.directory;
            let article_path = content_base_dir.join(topic_path).join(slug);

            if article_path.exists() {
                let index_path = article_path.join("index.mdx");
                if index_path.exists() {
                    process_article(
                        &index_path,
                        topic_key,
                        slug,
                        options,
                        &mut all_stats,
                        &mut total_words,
                        &mut total_articles,
                        &mut total_drafts,
                        &mut tag_counts,
                    )?;
                    found = true;
                }
            }
        }

        if !found {
            return Err(anyhow::anyhow!("No article found with slug: {}", slug));
        }
    } else {
        // Process all content
        for (topic_key, topic_config) in &config.content.topics {
            // Skip if a specific topic is requested and it's not this one
            if options.topic.is_some() && options.topic.as_ref() != Some(topic_key) {
                continue;
            }

            let topic_path = &topic_config.directory;
            let topic_dir = content_base_dir.join(topic_path);

            // Skip if the topic directory doesn't exist
            if !topic_dir.exists() {
                continue;
            }

            // Find all subdirectories in the topic directory (article directories)
            let dirs = common_fs::find_dirs_with_depth(&topic_dir, 1, 1)?;

            for article_dir in dirs {
                let slug = article_dir
                    .file_name()
                    .and_then(|name| name.to_str())
                    .unwrap_or("");

                let index_path = article_dir.join("index.mdx");
                if index_path.exists() {
                    process_article(
                        &index_path,
                        topic_key,
                        slug,
                        options,
                        &mut all_stats,
                        &mut total_words,
                        &mut total_articles,
                        &mut total_drafts,
                        &mut tag_counts,
                    )?;
                }
            }
        }
    }

    // Sort the statistics
    match options.sort_by.as_str() {
        "date" => {
            let sort_stats = |a: &ContentStats, b: &ContentStats| {
                // Compare the published dates
                if a.published == "DRAFT" && b.published == "DRAFT" {
                    a.title.cmp(&b.title)
                } else if a.published == "DRAFT" {
                    std::cmp::Ordering::Less
                } else if b.published == "DRAFT" {
                    std::cmp::Ordering::Greater
                } else {
                    b.published.cmp(&a.published)
                }
            };

            let mut stats_vec = all_stats.iter().cloned().collect::<Vec<_>>();
            stats_vec.sort_by(sort_stats);
            return Ok((
                stats_vec,
                tag_counts,
                total_words,
                total_articles,
                total_drafts,
            ));
        }
        "words" => {
            all_stats.sort_by(|a, b| b.word_count.cmp(&a.word_count));
            return Ok((
                all_stats,
                tag_counts,
                total_words,
                total_articles,
                total_drafts,
            ));
        }
        "reading_time" => {
            all_stats.sort_by(|a, b| b.reading_time.cmp(&a.reading_time));
            return Ok((
                all_stats,
                tag_counts,
                total_words,
                total_articles,
                total_drafts,
            ));
        }
        _ => {
            // Default sort by date
            let sort_stats = |a: &ContentStats, b: &ContentStats| {
                // Compare the published dates
                if a.published == "DRAFT" && b.published == "DRAFT" {
                    a.title.cmp(&b.title)
                } else if a.published == "DRAFT" {
                    std::cmp::Ordering::Less
                } else if b.published == "DRAFT" {
                    std::cmp::Ordering::Greater
                } else {
                    b.published.cmp(&a.published)
                }
            };

            let mut stats_vec = all_stats.iter().cloned().collect::<Vec<_>>();
            stats_vec.sort_by(sort_stats);
            return Ok((
                stats_vec,
                tag_counts,
                total_words,
                total_articles,
                total_drafts,
            ));
        }
    }
}

/// Process a single article file and extract statistics
fn process_article(
    #[allow(clippy::too_many_arguments)] index_path: &Path,
    topic_key: &str,
    slug: &str,
    options: &StatsOptions,
    all_stats: &mut Vec<ContentStats>,
    total_words: &mut usize,
    total_articles: &mut usize,
    total_drafts: &mut usize,
    tag_counts: &mut HashMap<String, usize>,
) -> Result<()> {
    // Read the content file
    let content = common_fs::read_file(index_path)?;

    // Extract frontmatter and content
    let (frontmatter, content_text) = common_markdown::extract_frontmatter_and_content(&content)?;

    // Check if draft and skip if not including drafts
    let is_draft = frontmatter.is_draft.unwrap_or(false)
        || frontmatter
            .published_at
            .as_ref()
            .is_some_and(|p| p == "DRAFT");

    if is_draft && !options.include_drafts {
        return Ok(());
    }

    // Calculate statistics
    let stats = calculate_stats(&content_text, &frontmatter, topic_key, slug);

    // Update totals
    *total_words += stats.word_count;
    *total_articles += 1;

    if is_draft {
        *total_drafts += 1;
    }

    // Update tag counts
    for tag in &stats.tags {
        *tag_counts.entry(tag.clone()).or_insert(0) += 1;
    }

    // Add to list of stats
    all_stats.push(stats);

    Ok(())
}

/// Get statistics for a specific topic
///
/// This function calculates statistics for a specific topic.
///
/// # Parameters
///
/// * `config` - Application configuration
/// * `topic_key` - Topic key
/// * `topic_config` - Topic configuration
///
/// # Returns
///
/// Returns statistics for the topic
///
/// # Errors
///
/// Returns an error if the statistics cannot be calculated
fn get_topic_stats(
    config: &Config,
    topic_key: &str,
    topic_config: &TopicConfig,
) -> Result<TopicStats> {
    let mut stats = TopicStats {
        key: topic_key.to_string(),
        name: topic_config.name.clone(),
        article_count: 0,
        word_count: 0,
        draft_count: 0,
        published_count: 0,
    };

    // Get the topic directory
    let topic_dir = PathBuf::from(&config.content.base_dir).join(&topic_config.directory);

    // Check if the topic directory exists
    if !topic_dir.exists() {
        return Ok(stats);
    }

    // Find all article directories
    let article_dirs = std::fs::read_dir(&topic_dir)?
        .filter_map(Result::ok)
        .filter(|entry| entry.path().is_dir())
        .map(|entry| entry.path())
        .collect::<Vec<_>>();

    // Process each article directory
    for article_dir in article_dirs {
        // Check if the article has an index.md file
        let index_file = article_dir.join("index.md");

        if index_file.exists() {
            // Read the index file
            let content = common_fs::read_file(&index_file)?;

            // Extract frontmatter
            if let Ok((frontmatter, content)) =
                common_markdown::extract_frontmatter_and_content(&content)
            {
                // Count words
                let word_count = content.split_whitespace().count();

                // Update statistics
                stats.article_count += 1;
                stats.word_count += word_count;

                if frontmatter.is_draft.unwrap_or(false) {
                    stats.draft_count += 1;
                } else {
                    stats.published_count += 1;
                }
            }
        }
    }

    Ok(stats)
}

/// Get statistics for all content
///
/// This function calculates statistics for all content.
///
/// # Parameters
///
/// * `options` - Statistics options
///
/// # Returns
///
/// Returns statistics for all content
///
/// # Errors
///
/// Returns an error if the statistics cannot be calculated
pub fn get_content_stats(options: &StatsOptions) -> Result<ContentStats> {
    let config = common_config::load_config()?;
    let mut stats = ContentStats {
        title: "Content Statistics".to_string(),
        published: "".to_string(),
        word_count: 0,
        reading_time: 0,
        character_count: 0,
        paragraph_count: 0,
        sentence_count: 0,
        topic: "".to_string(),
        slug: "".to_string(),
        tags: Vec::new(),
        is_draft: false,
        total_articles: 0,
        total_words: 0,
        total_drafts: 0,
        total_published: 0,
        topics: Vec::new(),
    };

    // If a specific topic is requested, only get stats for that topic
    if let Some(topic_key) = &options.topic {
        if let Some(topic_config) = config.content.topics.get(topic_key) {
            let topic_stats = get_topic_stats(&config, topic_key, topic_config)?;

            // Update totals
            stats.total_articles += topic_stats.article_count;
            stats.total_words += topic_stats.word_count;
            stats.total_drafts += topic_stats.draft_count;
            stats.total_published += topic_stats.published_count;

            // Add topic stats
            stats.topics.push(topic_stats);
        } else {
            return Err(anyhow::anyhow!("Topic not found: {}", topic_key));
        }
    } else {
        // Get stats for all topics
        for (topic_key, topic_config) in &config.content.topics {
            let topic_stats = get_topic_stats(&config, topic_key, topic_config)?;

            // Update totals
            stats.total_articles += topic_stats.article_count;
            stats.total_words += topic_stats.word_count;
            stats.total_drafts += topic_stats.draft_count;
            stats.total_published += topic_stats.published_count;

            // Add topic stats
            stats.topics.push(topic_stats);
        }
    }

    Ok(stats)
}

// Function is unused, so we can remove or comment it out
// fn validate_draft_status(frontmatter: &Frontmatter) -> Result<()> {
//     if frontmatter.is_draft.unwrap_or(false) {
//         Ok(())
//     } else {
//         Ok(())
//     }
// }

// Test utility functions for exposing internal functionality in tests
#[cfg(test)]
pub mod test_utils {
    use super::*;
    use std::cmp::Ordering;

    /// Get the sort function for testing
    pub fn get_sort_function() -> Option<fn(&ContentStats, &ContentStats) -> Ordering> {
        Some(|a, b| {
            // Sort by published date (newest first)
            if a.published == "DRAFT" && b.published == "DRAFT" {
                a.title.cmp(&b.title)
            } else if a.published == "DRAFT" {
                Ordering::Less
            } else if b.published == "DRAFT" {
                Ordering::Greater
            } else {
                b.published.cmp(&a.published)
            }
        })
    }

    /// Extract metadata and content from markdown for testing
    pub fn extract_metadata_and_content(
        content: &str,
    ) -> (String, HashMap<String, String>, String) {
        // Simplified test version of the internal function
        let parts: Vec<&str> = content.split("---").collect();
        if parts.len() >= 3 {
            let frontmatter = parts[1].trim();
            let frontmatter_yaml =
                serde_yaml::from_str::<HashMap<String, String>>(frontmatter).unwrap_or_default();
            let title = frontmatter_yaml.get("title").cloned().unwrap_or_default();
            let content = parts[2..].join("---").trim().to_string();
            (title, frontmatter_yaml, content)
        } else {
            ("".to_string(), HashMap::new(), content.to_string())
        }
    }

    /// Create excerpt for testing
    pub fn create_excerpt(text: &str, query: &str, max_length: usize) -> String {
        // Simplified test version of the internal function
        let position = text.to_lowercase().find(&query.to_lowercase());
        if let Some(pos) = position {
            let start = if pos > max_length / 2 {
                pos - max_length / 2
            } else {
                0
            };
            let end = std::cmp::min(start + max_length, text.len());
            let mut excerpt = text[start..end].to_string();
            if start > 0 {
                excerpt = format!("...{}", excerpt);
            }
            if end < text.len() {
                excerpt = format!("{}...", excerpt);
            }
            excerpt
        } else {
            if text.len() <= max_length {
                text.to_string()
            } else {
                format!("{}...", &text[0..max_length])
            }
        }
    }
}
