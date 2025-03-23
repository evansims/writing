/// Migrate content from the old format to the new format
///
/// This function migrates content from the old format to the new format.
///
/// # Parameters
///
/// * `options` - Migration options
///
/// # Returns
///
/// Returns a list of migration results
///
/// # Errors
///
/// Returns an error if the migration fails
pub fn migrate_content(options: &MigrationOptions) -> Result<Vec<MigrationResult>> {
    let config = load_config()?;
    let mut results = Vec::new();

    if let Some(topic) = &options.topic {
        // Migrate a specific topic
        if let Some(topic_config) = config.content.topics.get(topic) {
            let topic_dir = PathBuf::from(&config.content.base_dir).join(&topic_config.directory);
            if !topic_dir.exists() {
                return Err(MigrationError::TopicNotFound(topic.clone()).into());
            }

            let entries = std::fs::read_dir(&topic_dir)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                if path.is_dir() {
                    let slug = path
                        .file_name()
                        .and_then(|name| name.to_str())
                        .unwrap_or("")
                        .to_string();

                    if slug.is_empty() {
                        continue;
                    }

                    // Check for the old format index files
                    let index_file = path.join("index.md");
                    let index_file_mdx = path.join("index.mdx");

                    // Determine which index file exists
                    let index_file_option = if index_file.exists() {
                        Some(index_file)
                    } else if index_file_mdx.exists() {
                        Some(index_file_mdx)
                    } else {
                        None
                    };

                    if let Some(index_file) = index_file_option {
                        // Create the new file path
                        let extension = index_file
                            .extension()
                            .and_then(|ext| ext.to_str())
                            .unwrap_or("md");
                        let new_file = path.join(format!("{}.{}", slug, extension));

                        // Migration is needed
                        if !options.dry_run {
                            // Read the content from the index file
                            let content = read_file(&index_file)?;

                            // Write the content to the new file
                            write_file(&new_file, &content)?;

                            // Delete the old file if requested
                            if options.delete_old {
                                std::fs::remove_file(&index_file)?;
                            }
                        }

                        // Add the result
                        results.push(MigrationResult {
                            topic: topic.clone(),
                            slug,
                            from: index_file,
                            to: new_file,
                            success: !options.dry_run,
                        });
                    }
                }
            }
        } else {
            return Err(MigrationError::TopicNotFound(topic.clone()).into());
        }
    } else {
        // Migrate all topics
        for (topic_key, topic_config) in &config.content.topics {
            let topic_dir = PathBuf::from(&config.content.base_dir).join(&topic_config.directory);
            if !topic_dir.exists() {
                if options.verbose {
                    eprintln!("Topic directory not found: {}", topic_dir.display());
                }
                continue;
            }

            let entries = std::fs::read_dir(&topic_dir)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                if path.is_dir() {
                    let slug = path
                        .file_name()
                        .and_then(|name| name.to_str())
                        .unwrap_or("")
                        .to_string();

                    if slug.is_empty() {
                        continue;
                    }

                    // Check for the old format index files
                    let index_file = path.join("index.md");
                    let index_file_mdx = path.join("index.mdx");

                    // Determine which index file exists
                    let index_file_option = if index_file.exists() {
                        Some(index_file)
                    } else if index_file_mdx.exists() {
                        Some(index_file_mdx)
                    } else {
                        None
                    };

                    if let Some(index_file) = index_file_option {
                        // Create the new file path
                        let extension = index_file
                            .extension()
                            .and_then(|ext| ext.to_str())
                            .unwrap_or("md");
                        let new_file = path.join(format!("{}.{}", slug, extension));

                        // Migration is needed
                        if !options.dry_run {
                            // Read the content from the index file
                            let content = read_file(&index_file)?;

                            // Write the content to the new file
                            write_file(&new_file, &content)?;

                            // Delete the old file if requested
                            if options.delete_old {
                                std::fs::remove_file(&index_file)?;
                            }
                        }

                        // Add the result
                        results.push(MigrationResult {
                            topic: topic_key.clone(),
                            slug,
                            from: index_file,
                            to: new_file,
                            success: !options.dry_run,
                        });
                    }
                }
            }
        }
    }

    Ok(results)
}
