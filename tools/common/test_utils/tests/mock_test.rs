use common_test_utils::mocks::{MockFileSystem, MockConfigLoader, MockContentOperations, MockCommandExecutor};
use common_test_utils::mocks::traits::{FileSystem, ConfigLoader, CommandExecutor};
use common_models::{Article, Frontmatter, Config, ContentConfig, PublicationConfig};
use std::collections::HashMap;
use std::path::Path;

#[test]
fn test_mock_filesystem() {
    let mut mock_fs = MockFileSystem::new();
    
    // Add some files and directories
    mock_fs.add_file("/content/blog/post.md", "# Test Post\n\nContent");
    
    // Use trait methods for the rest of the operations
    let fs: &dyn FileSystem = &mock_fs;
    
    // Test file exists
    assert!(fs.file_exists(Path::new("/content/blog/post.md")));
    
    // Test directory exists
    assert!(fs.dir_exists(Path::new("/content/blog")));
    
    // Test reading file
    let content = fs.read_file(Path::new("/content/blog/post.md")).unwrap();
    assert_eq!(content, "# Test Post\n\nContent");
    
    // Test writing file
    fs.write_file(Path::new("/content/blog/post2.md"), "# Test Post 2".to_string()).unwrap();
    assert!(fs.file_exists(Path::new("/content/blog/post2.md")));
    
    // Test deleting file
    fs.delete_file(Path::new("/content/blog/post.md")).unwrap();
    assert!(!fs.file_exists(Path::new("/content/blog/post.md")));
    
    // Test listing files
    let files = fs.list_files(Path::new("/content/blog")).unwrap();
    assert_eq!(files.len(), 1);
    assert_eq!(files[0], "post2.md");
}

#[test]
fn test_mock_config_loader() {
    // Create a simple config
    let config = Config {
        content: ContentConfig {
            base_dir: "/content".to_string(),
            topics: HashMap::new(),
            tags: None,
        },
        images: common_models::ImageConfig {
            formats: vec!["jpg".to_string()],
            format_descriptions: None,
            sizes: HashMap::new(),
            naming: None,
            quality: None,
        },
        publication: PublicationConfig {
            author: "Test Author".to_string(),
            copyright: "Test Copyright".to_string(),
            site: None,
        },
    };
    
    let mut mock_config = MockConfigLoader::new(config.clone());
    
    // Test loading config
    let loaded_config = mock_config.load_config("/config.yaml").unwrap();
    assert_eq!(loaded_config.publication.author, "Test Author");
    
    // Test updating config
    let mut new_config = config.clone();
    new_config.publication.author = "New Author".to_string();
    mock_config.set_config(new_config);
    
    let loaded_config = mock_config.load_config("/config.yaml").unwrap();
    assert_eq!(loaded_config.publication.author, "New Author");
}

#[test]
fn test_mock_content_operations() {
    let mut mock_content = MockContentOperations::new();
    
    // Create a test article
    let article = Article {
        frontmatter: Frontmatter {
            title: "Test Article".to_string(),
            published: Some("2023-01-01".to_string()),
            updated: None,
            slug: Some("test-article".to_string()),
            tagline: None,
            tags: Some(vec!["test".to_string()]),
            topics: Some(vec!["blog".to_string()]),
            draft: Some(false),
            featured_image: None,
        },
        content: "# Test Article\n\nThis is a test article.".to_string(),
        slug: "test-article".to_string(),
        topic: "blog".to_string(),
        path: "/content/blog/test-article".to_string(),
        word_count: Some(7),
        reading_time: Some(1),
    };
    
    // Add the article
    mock_content.add_article(article.clone());
    
    // Test getting the article
    let retrieved = mock_content.get_article("blog", "test-article").unwrap();
    assert_eq!(retrieved.frontmatter.title, "Test Article");
    
    // Test listing articles
    let articles = mock_content.list_articles();
    assert_eq!(articles.len(), 1);
    
    // Test deleting the article
    mock_content.delete_article("blog", "test-article").unwrap();
    assert!(mock_content.get_article("blog", "test-article").is_none());
}

#[test]
fn test_mock_command_executor() {
    let mut mock_cmd = MockCommandExecutor::new();
    
    // Set up command responses
    mock_cmd.set_response("ls -la", "file1.txt\nfile2.txt", 0);
    mock_cmd.set_response("rm file1.txt", "", 0);
    mock_cmd.set_response("rm nonexistent.txt", "No such file or directory", 1);
    
    // Test executing commands
    let (output, exit_code) = mock_cmd.execute("ls -la").unwrap();
    assert_eq!(output, "file1.txt\nfile2.txt");
    assert_eq!(exit_code, 0);
    
    let (output, exit_code) = mock_cmd.execute("rm file1.txt").unwrap();
    assert_eq!(output, "");
    assert_eq!(exit_code, 0);
    
    let (output, exit_code) = mock_cmd.execute("rm nonexistent.txt").unwrap();
    assert_eq!(output, "No such file or directory");
    assert_eq!(exit_code, 1);
    
    // Test getting executed commands
    let commands = mock_cmd.get_executed_commands();
    assert_eq!(commands.len(), 3);
    assert_eq!(commands[0], "ls -la");
    assert_eq!(commands[1], "rm file1.txt");
    assert_eq!(commands[2], "rm nonexistent.txt");
    
    // Test clearing executed commands
    mock_cmd.clear_executed_commands();
    assert_eq!(mock_cmd.get_executed_commands().len(), 0);
}

#[test]
fn test_trait_implementations() {
    // Test FileSystem trait
    let fs: Box<dyn FileSystem> = Box::new(MockFileSystem::new());
    fs.write_file(std::path::Path::new("/test.txt"), "test content".to_string()).unwrap();
    assert!(fs.file_exists(std::path::Path::new("/test.txt")));
    assert_eq!(fs.read_file(std::path::Path::new("/test.txt")).unwrap(), "test content");
    
    // Test ConfigLoader trait
    let config = Config {
        content: ContentConfig {
            base_dir: "/content".to_string(),
            topics: HashMap::new(),
            tags: None,
        },
        images: common_models::ImageConfig {
            formats: vec!["jpg".to_string()],
            format_descriptions: None,
            sizes: HashMap::new(),
            naming: None,
            quality: None,
        },
        publication: PublicationConfig {
            author: "Test Author".to_string(),
            copyright: "Test Copyright".to_string(),
            site: None,
        },
    };
    
    let config_loader: Box<dyn ConfigLoader> = Box::new(MockConfigLoader::new(config));
    let loaded_config = config_loader.load_config(std::path::Path::new("/config.yaml")).unwrap();
    assert_eq!(loaded_config.publication.author, "Test Author");
    
    // Test CommandExecutor trait
    let mut cmd_executor = MockCommandExecutor::new();
    cmd_executor.set_response("test command", "test output", 0);
    
    let executor: Box<dyn CommandExecutor> = Box::new(cmd_executor);
    let (output, exit_code) = executor.execute("test command").unwrap();
    assert_eq!(output, "test output");
    assert_eq!(exit_code, 0);
} 