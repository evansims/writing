use anyhow::Result;
use clap::{Parser, ValueEnum};
use colored::*;
use content_validate::{
    ValidationOptions, ValidationType, ValidationIssueType,
    validate_content
};
use std::path::PathBuf;

#[derive(Copy, Clone, Debug, PartialEq, Eq, ValueEnum)]
enum ValidationTypeArg {
    Links,
    Markdown,
    All,
}

impl ValidationTypeArg {
    fn to_validation_type(&self) -> ValidationType {
        match self {
            ValidationTypeArg::Links => ValidationType::Links,
            ValidationTypeArg::Markdown => ValidationType::Markdown,
            ValidationTypeArg::All => ValidationType::All,
        }
    }
}

#[derive(Parser)]
#[command(author, version, about = "Validate content for links, markdown formatting, and spelling")]
struct Args {
    /// Article slug to validate
    #[arg(short, long)]
    article: Option<String>,

    /// Topic to validate
    #[arg(short, long)]
    topic: Option<String>,
    
    /// Types of validation to perform
    #[arg(short, long, value_enum, default_value = "all")]
    validation_types: Vec<ValidationTypeArg>,
    
    /// Skip external link checking
    #[arg(long, default_value = "false")]
    skip_external_links: bool,
    
    /// Timeout for external link checking (in seconds)
    #[arg(long, default_value = "10")]
    external_link_timeout: u64,
    
    /// Custom dictionary file path
    #[arg(long)]
    dictionary: Option<PathBuf>,
    
    /// Include draft content
    #[arg(long, default_value = "false")]
    include_drafts: bool,
    
    /// Display verbose output
    #[arg(short, long, default_value = "false")]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();
    
    // Convert validation types
    let validation_types = args.validation_types.iter()
        .map(|vt| vt.to_validation_type())
        .collect::<Vec<_>>();
    
    // Create validation options
    let options = ValidationOptions {
        article_slug: args.article.clone(),
        topic: args.topic.clone(),
        validation_types: validation_types.clone(),
        check_external_links: !args.skip_external_links,
        timeout: Some(args.external_link_timeout),
        dictionary_path: args.dictionary.clone(),
        include_drafts: args.include_drafts,
    };
    
    // Describe what we're doing
    if let Some(article) = &options.article_slug {
        println!("{} article: {}", "Validating".green().bold(), article);
    } else if let Some(topic) = &options.topic {
        println!("{} all articles in topic: {}", "Validating".green().bold(), topic);
    } else {
        println!("{} all content", "Validating".green().bold());
    }
    
    // Display validation options
    let validation_types_str: Vec<String> = options.validation_types.iter()
        .map(|vt| format!("{:?}", vt))
        .collect();
    println!("  {} {}", "Validation types:".cyan().bold(), validation_types_str.join(", "));
    
    if options.validation_types.iter().any(|&vt| vt == ValidationType::Links || vt == ValidationType::All) {
        println!("  {} {}", "Check external links:".cyan().bold(), options.check_external_links);
        if options.check_external_links {
            println!("  {} {} seconds", "External link timeout:".cyan().bold(), options.timeout.unwrap());
        }
    }
    
    if options.validation_types.iter().any(|&vt| vt == ValidationType::Markdown || vt == ValidationType::All) {
        if let Some(dict) = &options.dictionary_path {
            println!("  {} {}", "Custom dictionary:".cyan().bold(), dict.display());
        }
    }
    
    println!("  {} {}", "Include drafts:".cyan().bold(), options.include_drafts);
    
    // Run validation
    println!("\n{} content validation...", "Running".yellow().bold());
    let results = validate_content(&options)?;
    
    if results.is_empty() {
        println!("\n{} No content found to validate.", "Notice:".yellow().bold());
        return Ok(());
    }
    
    // Process results
    let mut total_issues = 0;
    let mut files_with_issues = 0;
    
    // Count issues by type
    let mut link_issues = 0;
    let mut markdown_issues = 0;
    
    for result in &results {
        if !result.issues.is_empty() {
            files_with_issues += 1;
            total_issues += result.issues.len();
            
            // Display file path
            println!("\n{} {}", "File:".blue().bold(), result.file_path.display());
            
            // Show issues
            for issue in &result.issues {
                match issue.issue_type {
                    ValidationIssueType::BrokenLink | 
                    ValidationIssueType::MissingInternalLink | 
                    ValidationIssueType::InvalidUrl => {
                        link_issues += 1;
                        println!("  {}: {}", "LINK".red().bold(), issue.description);
                    },
                    ValidationIssueType::MarkdownFormatting => {
                        markdown_issues += 1;
                        
                        if let Some(line) = issue.line {
                            println!("  {} (line {}): {}", "FORMAT".yellow().bold(), line, issue.description);
                        } else {
                            println!("  {}: {}", "FORMAT".yellow().bold(), issue.description);
                        }
                    },
                }
                
                // Show suggestion if available and verbose is enabled
                if args.verbose {
                    if let Some(suggestion) = &issue.suggested_fix {
                        println!("    Suggestion: {}", suggestion);
                    }
                }
            }
        }
    }
    
    // Summary
    println!("\n{}", "=== Validation Summary ===".green().bold());
    println!("Files checked: {}", results.len());
    println!("Files with issues: {}", files_with_issues);
    println!("Total issues: {}", total_issues);
    
    if validation_types.contains(&ValidationType::Links) || 
       validation_types.contains(&ValidationType::All) {
        println!("Link issues: {}", link_issues);
    }
    
    if validation_types.contains(&ValidationType::Markdown) || 
       validation_types.contains(&ValidationType::All) {
        println!("Markdown formatting issues: {}", markdown_issues);
    }
    
    if total_issues > 0 {
        println!("\n{} {} validation issues found", "Warning:".yellow().bold(), total_issues);
    } else {
        println!("\n{} No validation issues found!", "Success:".green().bold());
    }
    
    Ok(())
} 