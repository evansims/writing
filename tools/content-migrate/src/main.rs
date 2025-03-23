use anyhow::Result;
use clap::Parser;
use common_config::load_config;
use common_fs::{join_paths, read_file, write_file};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about = "Migrate content from old index.md format to new slug-named format",
    long_about = "A tool to migrate content from the old index.md/index.mdx format to the new slug/slug.md format"
)]
struct Args {
    /// Topic to migrate (if not specified, all topics will be migrated)
    #[arg(short, long)]
    topic: Option<String>,

    /// Dry run (don't actually modify files)
    #[arg(short = 'd', long)]
    dry_run: bool,

    /// Delete old index files after migration
    #[arg(short, long)]
    delete_old: bool,

    /// Verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let config = load_config()?;
    let topics = if let Some(topic) = &args.topic {
        if !config.content.topics.contains_key(topic) {
            return Err(anyhow::anyhow!("Topic not found: {}", topic));
        }
        vec![topic.clone()]
    } else {
        config.content.topics.keys().cloned().collect()
    };

    let mut migrated_count = 0;
    let mut skipped_count = 0;
    let mut error_count = 0;
    let mut deleted_count = 0;

    for topic in topics {
        let topic_config = &config.content.topics[&topic];
        let topic_dir = PathBuf::from(&config.content.base_dir).join(&topic_config.directory);

        if !topic_dir.exists() {
            if args.verbose {
                println!("Topic directory not found: {}", topic_dir.display());
            }
            continue;
        }

        // Find all content directories in the topic
        for entry in fs::read_dir(&topic_dir)? {
            let entry = entry?;
            let content_dir = entry.path();

            if !content_dir.is_dir() {
                continue;
            }

            let slug = content_dir
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or("")
                .to_string();

            if slug.is_empty() {
                continue;
            }

            // Check if this directory contains an index.md or index.mdx file
            let index_md = content_dir.join("index.md");
            let index_mdx = content_dir.join("index.mdx");

            // Check if the new format file already exists
            let new_format_md = content_dir.join(format!("{}.md", slug));
            let new_format_mdx = content_dir.join(format!("{}.mdx", slug));

            if new_format_md.exists() || new_format_mdx.exists() {
                if args.verbose {
                    println!("Content already in new format: {}/{}", topic, slug);
                }
                skipped_count += 1;
                continue;
            }

            // If index.md exists, migrate it
            if index_md.exists() {
                if args.verbose {
                    println!("Migrating: {}/{}/index.md", topic, slug);
                }

                if !args.dry_run {
                    if let Err(e) = migrate_file(&index_md, &new_format_md) {
                        eprintln!("Error migrating {}: {}", index_md.display(), e);
                        error_count += 1;
                        continue;
                    }

                    // Delete the old file if requested
                    if args.delete_old {
                        if args.verbose {
                            println!("Deleting old file: {}", index_md.display());
                        }

                        if let Err(e) = fs::remove_file(&index_md) {
                            eprintln!("Error deleting {}: {}", index_md.display(), e);
                            error_count += 1;
                        } else {
                            deleted_count += 1;
                        }
                    }
                }
                migrated_count += 1;
            }
            // If index.mdx exists, migrate it
            else if index_mdx.exists() {
                if args.verbose {
                    println!("Migrating: {}/{}/index.mdx", topic, slug);
                }

                if !args.dry_run {
                    if let Err(e) = migrate_file(&index_mdx, &new_format_mdx) {
                        eprintln!("Error migrating {}: {}", index_mdx.display(), e);
                        error_count += 1;
                        continue;
                    }

                    // Delete the old file if requested
                    if args.delete_old {
                        if args.verbose {
                            println!("Deleting old file: {}", index_mdx.display());
                        }

                        if let Err(e) = fs::remove_file(&index_mdx) {
                            eprintln!("Error deleting {}: {}", index_mdx.display(), e);
                            error_count += 1;
                        } else {
                            deleted_count += 1;
                        }
                    }
                }
                migrated_count += 1;
            } else {
                if args.verbose {
                    println!("No index file found in: {}/{}", topic, slug);
                }
                skipped_count += 1;
            }
        }
    }

    println!("Migration complete!");
    println!("Migrated: {}", migrated_count);
    println!("Skipped: {}", skipped_count);
    println!("Deleted: {}", deleted_count);
    println!("Errors: {}", error_count);

    Ok(())
}

/// Migrate a file from the old format to the new format
fn migrate_file(old_path: &Path, new_path: &Path) -> Result<()> {
    // Read the old file
    let content = read_file(old_path)?;

    // Write to the new file
    write_file(new_path, &content)?;

    // We don't delete the old file yet to ensure backward compatibility
    // If you want to delete the old file, uncomment the following line:
    // fs::remove_file(old_path)?;

    Ok(())
}
