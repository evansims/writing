// Note: Due to parallel test execution and shared file access, these tests should be run individually
use std::fs;
use std::path::{Path, PathBuf};
use std::env;
use anyhow::Result;
use common_models::{Config, TopicConfig, ImageSize};
use common_test_utils::TestFixture;
use topic_add::{AddOptions, add_topic, generate_key_from_name};

/// Create directories and check permissions
fn setup_test_directories(fixture_path: &Path) -> Result<()> {
    // Create content directory
    let content_dir = fixture_path.join("content");
    println!("Creating content directory at: {:?}", content_dir);
    fs::create_dir_all(&content_dir)?;

    // Create blog directory (for duplicate topic test)
    let blog_dir = content_dir.join("blog");
    println!("Creating blog directory at: {:?}", blog_dir);
    fs::create_dir_all(&blog_dir)?;

    Ok(())
}

/// Create a config file with default values
fn create_test_config(fixture_path: &Path) -> Result<Config> {
    // Create a config with necessary values
    let mut config = Config::default();

    // Set content base directory
    config.content.base_dir = "content".to_string();

    // Set other required fields
    config.title = "Test Site".to_string();
    config.email = "test@example.com".to_string();
    config.url = "https://example.com".to_string();
    config.image = "https://example.com/image.jpg".to_string();
    config.default_topic = Some("blog".to_string());

    // Add a blog topic (for duplicate topic test)
    config.content.topics.insert(
        "blog".to_string(),
        TopicConfig {
            name: "Blog".to_string(),
            description: "Test blog".to_string(),
            directory: "blog".to_string(),
        },
    );

    // Set up image formats
    config.images.formats = vec!["jpg".to_string(), "png".to_string()];

    // Set up image sizes
    let mut sizes = std::collections::HashMap::new();
    sizes.insert(
        "featured".to_string(),
        ImageSize {
            width: 1200,
            height: 630,
            description: "Featured image".to_string(),
        },
    );
    sizes.insert(
        "social".to_string(),
        ImageSize {
            width: 1200,
            height: 630,
            description: "Social media image".to_string(),
        },
    );
    sizes.insert(
        "thumb".to_string(),
        ImageSize {
            width: 400,
            height: 300,
            description: "Thumbnail image".to_string(),
        },
    );
    config.images.sizes = sizes;

    // Set publication info
    config.publication.author = "Author Name".to_string();
    config.publication.copyright = "© 2023, All rights reserved".to_string();
    config.publication.site_url = Some("https://example.com".to_string());

    // Save the config to a file
    let config_path = fixture_path.join("config.yaml");
    println!("Saving config to: {:?}", config_path);
    let config_str = serde_yaml::to_string(&config)?;
    fs::write(&config_path, &config_str)?;

    // Verify the file was created
    assert!(config_path.exists(), "Failed to create config file");

    Ok(config)
}

fn run_test_in_directory<F, R>(dir: &Path, f: F) -> Result<R>
where
    F: FnOnce() -> Result<R>,
{
    // Store current directory
    let original_dir = env::current_dir()?;
    println!("Current working directory: {:?}", original_dir);

    // Change to test directory
    println!("Changing directory from {:?} to {:?}", original_dir, dir);
    env::set_current_dir(dir)?;

    // Set CONFIG_PATH environment variable to point to the config file in the current directory
    let config_path = env::current_dir()?.join("config.yaml");
    println!("Setting CONFIG_PATH environment variable to: {:?}", config_path);
    env::set_var("CONFIG_PATH", config_path.to_string_lossy().to_string());

    // Run the function
    let result = f();

    // If there's a config.yaml in the current directory, copy it back to the fixture path
    if Path::new("config.yaml").exists() {
        println!("Copying the updated config.yaml back to the fixture path");
        let config_content = fs::read_to_string("config.yaml")?;
        fs::write(dir.join("config.yaml"), config_content)?;
    }

    // Return to original directory
    println!("Resetting working directory to: {:?}", original_dir);
    env::set_current_dir(original_dir)?;

    // Unset the CONFIG_PATH environment variable
    env::remove_var("CONFIG_PATH");

    result
}

/// Simple test for adding a topic
#[test]
fn test_add_topic_simple() -> Result<()> {
    // Create fixture
    let fixture = TestFixture::new()?;
    let fixture_path = fixture.path().to_path_buf();
    println!("Created test fixture at: {:?}", fixture_path);

    // Setup directories
    setup_test_directories(&fixture_path)?;
    let config = create_test_config(&fixture_path)?;

    // Set environment variable to point to config
    let config_path = fixture_path.join("config.yaml");
    env::set_var("CONFIG_PATH", config_path.to_string_lossy().to_string());
    println!("Set CONFIG_PATH to: {:?}", config_path);

    // Test adding a topic
    let options = AddOptions {
        key: "tutorials".to_string(),
        name: "Tutorials".to_string(),
        description: "Programming tutorials".to_string(),
        directory: "tutorials".to_string(),
    };

    // Create directory before using add_topic
    let tutorial_dir = fixture_path.join("content").join("tutorials");
    println!("Creating tutorial directory: {:?}", tutorial_dir);
    fs::create_dir_all(&tutorial_dir)?;

    // Verify we're in the right directory
    println!("Current directory: {:?}", env::current_dir()?);
    println!("Fixture directory exists: {}", fixture_path.exists());
    println!("Config file exists: {}", config_path.exists());
    println!("Tutorial directory exists: {}", tutorial_dir.exists());

    // Add the topic
    println!("Adding topic: {}", options.key);
    let result = add_topic(&options)?;
    println!("Result: {}", result);

    // Test for success
    assert_eq!(result, "tutorials", "Topic key doesn't match");

    // Cleanup
    env::remove_var("CONFIG_PATH");

    Ok(())
}

/// Test adding a topic with manual directory verification
#[test]
fn test_topic_add_lifecycle() -> Result<()> {
    // Create the fixture
    let fixture = TestFixture::new()?;
    let fixture_path = fixture.path().to_path_buf();
    println!("Created test fixture at: {:?}", fixture_path);

    // Create directories
    setup_test_directories(&fixture_path)?;

    // Create config file
    let _config = create_test_config(&fixture_path)?;

    // Print config file path and check it exists
    let config_path = fixture_path.join("config.yaml");
    println!("Config file exists: {}", config_path.exists());
    assert!(config_path.exists(), "Config file does not exist");

    // Create topic directory manually
    let tutorials_dir = fixture_path.join("content").join("tutorials");
    println!("Creating topic directory manually: content/tutorials");
    fs::create_dir_all(&tutorials_dir)?;

    println!("Checking if directory exists: {:?}", tutorials_dir);
    assert!(tutorials_dir.exists(), "Directory does not exist");

    // Run the add_topic function within the fixture directory
    let result = run_test_in_directory(&fixture_path, || {
        // Verify the config file exists in current directory
        let config_file = Path::new("config.yaml");
        assert!(config_file.exists(), "Config file does not exist in test directory");
        println!("Current directory content:");
        if let Ok(entries) = fs::read_dir(".") {
            for entry in entries.flatten() {
                println!("  - {:?}", entry.path());
            }
        }

        // Print the content of the config file for debugging
        if let Ok(content) = fs::read_to_string("config.yaml") {
            println!("Config content before adding topic:\n{}", content);
        }

        // Create options
        let options = AddOptions {
            key: "tutorials".to_string(),
            name: "Tutorials".to_string(),
            description: "Programming tutorials".to_string(),
            directory: "tutorials".to_string(),
        };

        println!("Adding topic with options: key={}, name={}, directory={}",
            options.key, options.name, options.directory);

        // Add the topic
        let result = add_topic(&options);

        // Print the content of the config file after adding the topic
        if let Ok(content) = fs::read_to_string("config.yaml") {
            println!("Config content after adding topic:\n{}", content);
        }

        result
    })?;

    println!("Add topic result: {}", result);
    assert_eq!(result, "tutorials", "Result does not match expected key");

    // Verify the updated config file exists
    assert!(config_path.exists(), "Config file disappeared after update");

    // Safely read the config file
    let config_content = fs::read_to_string(&config_path)
        .map_err(|e| anyhow::anyhow!("Failed to read config after update: {}, path: {:?}", e, config_path))?;

    println!("Config content after operation:\n{}", config_content);

    // Parse the config
    let config = serde_yaml::from_str::<Config>(&config_content)
        .map_err(|e| anyhow::anyhow!("Failed to parse YAML: {}", e))?;

    // Verify the topic was added to the config
    println!("Topics in config: {:?}", config.content.topics.keys().collect::<Vec<_>>());
    println!("Verifying topic in config: {}", config.content.topics.contains_key("tutorials"));
    assert!(config.content.topics.contains_key("tutorials"), "Topic 'tutorials' was not found in the config");

    Ok(())
}

/// Test with key generation and manual directory creation
#[test]
fn test_integration_with_key_generation() -> Result<()> {
    // Create the fixture
    let fixture = TestFixture::new()?;
    let fixture_path = fixture.path().to_path_buf();
    println!("Created test fixture at: {:?}", fixture_path);

    // Create directories
    setup_test_directories(&fixture_path)?;

    // Create config file
    let _config = create_test_config(&fixture_path)?;

    // Generate a key from a name
    let name = "Advanced Tutorials & Tips";
    let key = generate_key_from_name(name);
    println!("Generated key '{}' from name '{}'", key, name);

    // Create the directory manually
    let topic_dir = fixture_path.join("content").join(&key);
    println!("Creating topic directory manually: content/{}", key);
    fs::create_dir_all(&topic_dir)?;

    println!("Checking if directory exists: {:?}", topic_dir);
    assert!(topic_dir.exists(), "Directory does not exist");

    // Run the add_topic function within the fixture directory
    let result = run_test_in_directory(&fixture_path, || {
        // Verify the config file exists in current directory
        let config_file = Path::new("config.yaml");
        assert!(config_file.exists(), "Config file does not exist in test directory");
        println!("Current directory content:");
        if let Ok(entries) = fs::read_dir(".") {
            for entry in entries.flatten() {
                println!("  - {:?}", entry.path());
            }
        }

        // Print the content of the config file for debugging
        if let Ok(content) = fs::read_to_string("config.yaml") {
            println!("Config content before adding topic:\n{}", content);
        }

        let options = AddOptions {
            key: key.clone(),
            name: name.to_string(),
            description: "Advanced programming tutorials and tips".to_string(),
            directory: key.clone(),
        };

        println!("Adding topic with options: key={}, name={}, directory={}",
            options.key, options.name, options.directory);

        // Add the topic
        let result = add_topic(&options);

        // Print the content of the config file after adding the topic
        if let Ok(content) = fs::read_to_string("config.yaml") {
            println!("Config content after adding topic:\n{}", content);
        }

        result
    })?;

    println!("Add topic result: {}", result);
    assert_eq!(result, key, "Result does not match expected key");

    // Verify the updated config file exists
    let config_path = fixture_path.join("config.yaml");
    println!("Checking if config file exists after adding topic: {:?}", config_path);
    assert!(config_path.exists(), "Config file disappeared after update");

    // Safely read the config file
    let config_content = fs::read_to_string(&config_path)
        .map_err(|e| anyhow::anyhow!("Failed to read config after update: {}, path: {:?}", e, config_path))?;

    println!("Config content after operation:\n{}", config_content);

    // Parse the config
    let config = serde_yaml::from_str::<Config>(&config_content)
        .map_err(|e| anyhow::anyhow!("Failed to parse YAML: {}", e))?;

    // Verify the topic was added to the config
    println!("Topics in config: {:?}", config.content.topics.keys().collect::<Vec<_>>());
    println!("Verifying topic in config: {}", config.content.topics.contains_key(&key));
    assert!(config.content.topics.contains_key(&key), "Topic '{}' was not found in the config", key);

    Ok(())
}

/// Test attempting to add a duplicate topic
#[test]
fn test_add_duplicate_topic() -> Result<()> {
    // Arrange
    let fixture = TestFixture::new()?;
    let fixture_path = fixture.path();
    println!("Created test fixture at: {:?}", fixture_path);

    // Create directories
    setup_test_directories(fixture_path)?;

    // Create config file with existing blog topic
    let _config = create_test_config(fixture_path)?;

    // Run the add_topic function within the fixture directory
    let err = run_test_in_directory(fixture_path, || {
        println!("Attempting to add duplicate topic with key: blog");

        let options = AddOptions {
            key: "blog".to_string(),
            name: "Blog Duplicate".to_string(),
            description: "Duplicate blog topic".to_string(),
            directory: "blog-duplicate".to_string(),
        };

        add_topic(&options)
    }).unwrap_err();

    // Check the error message
    let error_message = err.to_string();
    println!("Error message: {}", error_message);
    assert!(error_message.contains("already exists"));

    // Verify the duplicate directory was not created
    let topic_dir = fixture_path.join("content").join("blog-duplicate");
    println!("Checking if duplicate directory was created: {:?}", topic_dir);
    println!("Duplicate directory exists: {}", topic_dir.exists());
    assert!(!topic_dir.exists());

    Ok(())
}

/// Simple test that uses the generate_key_from_name function
#[test]
fn test_add_topic_with_generated_key() -> Result<()> {
    // Create fixture
    let fixture = TestFixture::new()?;
    let fixture_path = fixture.path().to_path_buf();
    println!("Created test fixture at: {:?}", fixture_path);

    // Setup directories
    setup_test_directories(&fixture_path)?;
    let _config = create_test_config(&fixture_path)?;

    // Generate a key from a name
    let name = "Advanced Tutorials & Tips";
    let key = generate_key_from_name(name);
    println!("Generated key '{}' from name '{}'", key, name);

    // Set environment variable to point to config
    let config_path = fixture_path.join("config.yaml");
    env::set_var("CONFIG_PATH", config_path.to_string_lossy().to_string());
    println!("Set CONFIG_PATH to: {:?}", config_path);

    // Create the directory manually
    let topic_dir = fixture_path.join("content").join(&key);
    println!("Creating topic directory manually: content/{}", key);
    fs::create_dir_all(&topic_dir)?;

    println!("Checking if directory exists: {:?}", topic_dir);
    assert!(topic_dir.exists(), "Directory does not exist");

    // Test adding a topic
    let options = AddOptions {
        key: key.clone(),
        name: name.to_string(),
        description: "Advanced programming tutorials and tips".to_string(),
        directory: key.clone(),
    };

    // Verify we're in the right directory
    println!("Current directory: {:?}", env::current_dir()?);
    println!("Fixture directory exists: {}", fixture_path.exists());
    println!("Config file exists: {}", config_path.exists());
    println!("Topic directory exists: {}", topic_dir.exists());

    // Add the topic
    println!("Adding topic with generated key: {}", options.key);
    let result = add_topic(&options)?;
    println!("Result: {}", result);

    // Test for success
    assert_eq!(result, key, "Topic key doesn't match");

    // Read config file to verify topic was added
    let config_content = fs::read_to_string(&config_path)?;
    let config: Config = serde_yaml::from_str(&config_content)?;

    // Verify the topic was added to the config
    println!("Topics in config: {:?}", config.content.topics.keys().collect::<Vec<_>>());
    println!("Verifying topic in config: {}", config.content.topics.contains_key(&key));
    assert!(config.content.topics.contains_key(&key), "Topic '{}' was not found in the config", key);

    // Cleanup
    env::remove_var("CONFIG_PATH");

    Ok(())
}

/// Super simple test that just changes directory before running add_topic
#[test]
fn test_add_topic_with_cd() -> Result<()> {
    // Create fixture
    let fixture = TestFixture::new()?;
    let fixture_path = fixture.path().to_path_buf();
    println!("Created test fixture at: {:?}", fixture_path);

    // Setup directories
    setup_test_directories(&fixture_path)?;
    let _config = create_test_config(&fixture_path)?;

    // Save original directory
    let original_dir = env::current_dir()?;
    println!("Original directory: {:?}", original_dir);

    // Create a directory for the topic
    let tutorial_dir = fixture_path.join("content").join("tutorials");
    println!("Creating tutorial directory: {:?}", tutorial_dir);
    fs::create_dir_all(&tutorial_dir)?;

    // Change to the fixture directory
    println!("Changing to fixture directory: {:?}", fixture_path);
    env::set_current_dir(&fixture_path)?;

    // Verify config exists in current directory
    let config_path = Path::new("config.yaml");
    println!("Config file exists: {}", config_path.exists());
    assert!(config_path.exists());

    // Print the initial config content
    let initial_config_content = fs::read_to_string(config_path)?;
    println!("Initial config content:\n{}", initial_config_content);

    // Create options
    let options = AddOptions {
        key: "tutorials".to_string(),
        name: "Programming Tutorials".to_string(),
        description: "Programming tutorials and tips".to_string(),
        directory: "tutorials".to_string(),
    };

    // Add the topic
    println!("Adding topic: {}", options.key);
    let result = add_topic(&options)?;
    println!("Result: {}", result);

    // Verify it was added correctly
    assert_eq!(result, "tutorials", "Result doesn't match expected key");

    // Read config file to verify topic was added
    let config_content = fs::read_to_string(config_path)?;
    println!("Updated config content:\n{}", config_content);
    let config: Config = serde_yaml::from_str(&config_content)?;

    // Verify the topic was added to the config
    println!("Topics in config: {:?}", config.content.topics.keys().collect::<Vec<_>>());
    println!("Verifying topic in config: {}", config.content.topics.contains_key("tutorials"));
    assert!(config.content.topics.contains_key("tutorials"), "Topic 'tutorials' was not found in the config");

    // Change back to original directory
    println!("Changing back to original directory: {:?}", original_dir);
    env::set_current_dir(original_dir)?;

    Ok(())
}

/// Super simple test that just changes directory before running add_topic with key generation
#[test]
fn test_add_topic_with_key_generation_and_cd() -> Result<()> {
    // Create fixture
    let fixture = TestFixture::new()?;
    let fixture_path = fixture.path().to_path_buf();
    println!("Created test fixture at: {:?}", fixture_path);

    // Setup directories
    setup_test_directories(&fixture_path)?;
    let _config = create_test_config(&fixture_path)?;

    // Save original directory
    let original_dir = env::current_dir()?;
    println!("Original directory: {:?}", original_dir);

    // Generate key
    let name = "Reference Materials";
    let key = generate_key_from_name(name);
    println!("Generated key '{}' from name '{}'", key, name);

    // Create a directory for the topic
    let topic_dir = fixture_path.join("content").join(&key);
    println!("Creating topic directory: {:?}", topic_dir);
    fs::create_dir_all(&topic_dir)?;

    // Change to the fixture directory
    println!("Changing to fixture directory: {:?}", fixture_path);
    env::set_current_dir(&fixture_path)?;

    // Verify config exists in current directory
    let config_path = Path::new("config.yaml");
    println!("Config file exists: {}", config_path.exists());
    assert!(config_path.exists());

    // Print the initial config content
    let initial_config_content = fs::read_to_string(config_path)?;
    println!("Initial config content:\n{}", initial_config_content);

    // Create options
    let options = AddOptions {
        key: key.clone(),
        name: name.to_string(),
        description: "Reference materials and documentation".to_string(),
        directory: key.clone(),
    };

    // Add the topic
    println!("Adding topic: {}", options.key);
    let result = add_topic(&options)?;
    println!("Result: {}", result);

    // Verify it was added correctly
    assert_eq!(result, key, "Result doesn't match expected key");

    // Read config file to verify topic was added
    let config_content = fs::read_to_string(config_path)?;
    println!("Updated config content:\n{}", config_content);
    let config: Config = serde_yaml::from_str(&config_content)?;

    // Verify the topic was added to the config
    println!("Topics in config: {:?}", config.content.topics.keys().collect::<Vec<_>>());
    println!("Verifying topic in config: {}", config.content.topics.contains_key(&key));
    assert!(config.content.topics.contains_key(&key), "Topic '{}' was not found in the config", key);

    // Change back to original directory
    println!("Changing back to original directory: {:?}", original_dir);
    env::set_current_dir(original_dir)?;

    Ok(())
}