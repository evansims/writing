//! # Mock Content Operations Implementation
//! 
//! This module provides a mock implementation of content operations for testing.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use common_errors::{Result, WritingError};
use common_models::Article;

/// A mock implementation of content operations
#[derive(Debug, Clone, Default)]
pub struct MockContentOperations {
    articles: Arc<Mutex<HashMap<String, Article>>>,
}

impl MockContentOperations {
    /// Create a new mock content operations implementation
    pub fn new() -> Self {
        Self {
            articles: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    
    /// Add an article to the mock content store
    pub fn add_article(&mut self, article: Article) {
        let key = format!("{}:{}", article.topic, article.slug);
        self.articles.lock().unwrap().insert(key, article);
    }
    
    /// Get an article from the mock content store
    pub fn get_article(&self, topic: &str, slug: &str) -> Option<Article> {
        let key = format!("{}:{}", topic, slug);
        let articles = self.articles.lock().unwrap();
        articles.get(&key).cloned()
    }
    
    /// Delete an article from the mock content store
    pub fn delete_article(&mut self, topic: &str, slug: &str) -> Result<()> {
        let key = format!("{}:{}", topic, slug);
        let mut articles = self.articles.lock().unwrap();
        
        if articles.remove(&key).is_some() {
            Ok(())
        } else {
            Err(WritingError::content_not_found(slug))
        }
    }
    
    /// List all articles in the mock content store
    pub fn list_articles(&self) -> Vec<Article> {
        let articles = self.articles.lock().unwrap();
        articles.values().cloned().collect()
    }
}

/// Trait for content operations
pub trait ContentOperations {
    /// Get an article
    fn get_article(&self, topic: &str, slug: &str) -> Option<Article>;
    
    /// List all articles
    fn list_articles(&self) -> Vec<Article>;
    
    /// Delete an article
    fn delete_article(&mut self, topic: &str, slug: &str) -> Result<()>;
}

// Implement the trait for the mock
impl ContentOperations for MockContentOperations {
    fn get_article(&self, topic: &str, slug: &str) -> Option<Article> {
        self.get_article(topic, slug)
    }
    
    fn list_articles(&self) -> Vec<Article> {
        self.list_articles()
    }
    
    fn delete_article(&mut self, topic: &str, slug: &str) -> Result<()> {
        self.delete_article(topic, slug)
    }
} 