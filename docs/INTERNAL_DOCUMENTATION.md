# Internal Function Documentation

This document provides detailed documentation for complex internal functions in the Writing project. These functions are not part of the public API but are important for understanding the codebase.

## Table of Contents

1. [Content Validation](#content-validation)
   - [is_draft_content](#is_draft_content)
   - [get_internal_resources](#get_internal_resources)
   - [load_custom_dictionary](#load_custom_dictionary)
   - [init_spell_checker](#init_spell_checker)
   - [validate_links](#validate_links)
   - [validate_markdown_formatting](#validate_markdown_formatting)
   - [validate_spelling](#validate_spelling)
   - [validate_content](#validate_content)

## Content Validation

The content validation module provides functionality for validating content files, including checking links, markdown formatting, and spelling.

### is_draft_content

```rust
fn is_draft_content(file_path: &Path) -> Result<bool>
```

Determines if a content file is marked as a draft.

#### Parameters

- `file_path` - Path to the content file to check

#### Returns

- `Result<bool>` - `true` if the content is marked as a draft, `false` otherwise

#### Implementation Details

This function reads the content file and performs a simple check for the presence of `draft: true` in the frontmatter. A more robust implementation would use proper YAML parsing.

#### Example

```rust
let is_draft = is_draft_content(Path::new("content/blog/my-post/index.mdx"))?;
if is_draft {
    println!("This post is still a draft");
}
```

### get_internal_resources

```rust
fn get_internal_resources(config: &Config) -> Result<HashSet<String>>
```

Collects all internal resources (files, images, etc.) in the content directory.

#### Parameters

- `config` - The application configuration

#### Returns

- `Result<HashSet<String>>` - A set of site-relative paths to all internal resources

#### Implementation Details

This function:
1. Iterates through all topics in the configuration
2. For each topic, finds all files in the topic directory
3. Converts absolute paths to site-relative paths
4. Also includes files in the build directory if it exists

The resulting set is used for validating internal links to ensure they point to existing resources.

#### Example

```rust
let config = load_config()?;
let resources = get_internal_resources(&config)?;

// Check if a specific resource exists
if resources.contains("blog/my-post/image.jpg") {
    println!("The image exists");
}
```

### load_custom_dictionary

```rust
fn load_custom_dictionary(path: &Path) -> Result<HashSet<String>>
```

Loads a custom dictionary from a file for spell checking.

#### Parameters

- `path` - Path to the dictionary file

#### Returns

- `Result<HashSet<String>>` - A set of words from the dictionary

#### Implementation Details

This function:
1. Reads the dictionary file
2. Splits the content by lines
3. Trims each line and converts to lowercase
4. Filters out empty lines
5. Returns a HashSet of words

The dictionary file should contain one word per line.

#### Example

```rust
let custom_dict = load_custom_dictionary(Path::new("dictionaries/technical.txt"))?;
println!("Loaded {} custom words", custom_dict.len());
```

### init_spell_checker

```rust
fn init_spell_checker(custom_dict_path: Option<&Path>) -> Result<SymSpell>
```

Initializes a spell checker with dictionaries.

#### Parameters

- `custom_dict_path` - Optional path to a custom dictionary file

#### Returns

- `Result<SymSpell>` - An initialized spell checker

#### Implementation Details

This function:
1. Creates a new SymSpell instance with appropriate configuration
2. Loads the default English dictionary
3. If a custom dictionary is provided, loads it and adds its words to the spell checker
4. Returns the configured spell checker

The spell checker is used for validating spelling in content files.

#### Example

```rust
let spell_checker = init_spell_checker(Some(Path::new("dictionaries/technical.txt")))?;
let suggestions = spell_checker.lookup("programing", Verbosity::Top);
println!("Did you mean: {}", suggestions[0].term);  // "programming"
```

### validate_links

```rust
fn validate_links(
    content: &str,
    file_path: &Path,
    resources: &HashSet<String>,
    check_external: bool,
    timeout: u64,
) -> Vec<ValidationIssue>
```

Validates links in content to ensure they point to valid resources.

#### Parameters

- `content` - The content to validate
- `file_path` - Path to the content file (used for resolving relative links)
- `resources` - Set of internal resources to check against
- `check_external` - Whether to check external links
- `timeout` - Timeout in seconds for external link checking

#### Returns

- `Vec<ValidationIssue>` - List of validation issues found

#### Implementation Details

This function:
1. Extracts links from markdown content
2. Also uses LinkFinder to catch any links the markdown parser might miss
3. For external links (starting with http:// or https://):
   - If `check_external` is true, sends a HEAD request to verify the link
   - Reports broken links as validation issues
4. For internal links:
   - Normalizes the link path
   - Checks if the resource exists in the `resources` set
   - Reports missing resources as validation issues

The function handles both absolute and relative internal links, as well as external links.

#### Example

```rust
let content = "Check out [this link](/blog/my-post) and [this external link](https://example.com)";
let file_path = Path::new("content/blog/current-post/index.mdx");
let resources = get_internal_resources(&config)?;

let issues = validate_links(content, file_path, &resources, true, 10);
for issue in issues {
    println!("Link issue: {}", issue.description);
}
```

### validate_markdown_formatting

```rust
fn validate_markdown_formatting(content: &str) -> Vec<ValidationIssue>
```

Validates markdown formatting to ensure it follows best practices.

#### Parameters

- `content` - The markdown content to validate

#### Returns

- `Vec<ValidationIssue>` - List of validation issues found

#### Implementation Details

This function checks for several markdown formatting issues:

1. **Heading Hierarchy**: Ensures headings don't skip levels (e.g., H1 -> H3)
2. **Long Paragraphs**: Identifies very long paragraphs (>500 characters) that might be hard to read
3. **Consistent List Markers**: Checks that the document uses either `-` or `*` for lists, but not both
4. **Code Block Syntax**: Verifies that code blocks specify a language for syntax highlighting

The function uses a markdown parser to analyze the document structure and reports any issues found.

#### Example

```rust
let content = "# Heading 1\n\n### Heading 3\n\nSome text with a very long paragraph...";
let issues = validate_markdown_formatting(content);
for issue in issues {
    println!("Markdown issue: {}", issue.description);
    if let Some(suggestion) = issue.suggestion {
        println!("Suggestion: {}", suggestion);
    }
}
```

### validate_spelling

```rust
fn validate_spelling(content: &str, spell_checker: &SymSpell) -> Vec<ValidationIssue>
```

Validates spelling in content to identify potential spelling errors.

#### Parameters

- `content` - The content to check for spelling errors
- `spell_checker` - The initialized spell checker

#### Returns

- `Vec<ValidationIssue>` - List of spelling issues found

#### Implementation Details

This function:
1. Removes frontmatter and code blocks from the content
2. Strips HTML tags and markdown syntax
3. Splits the content into words
4. Checks each word against the spell checker
5. For words not found in the dictionary:
   - Generates suggestions for corrections
   - Reports spelling errors as validation issues

The function ignores:
- Words in frontmatter
- Words in code blocks
- Words shorter than 3 characters
- Words containing numbers

#### Example

```rust
let content = "This is a documnet with a speling error.";
let spell_checker = init_spell_checker(None)?;
let issues = validate_spelling(content, &spell_checker);
for issue in issues {
    println!("Spelling issue: {}", issue.description);
    if let Some(suggestion) = issue.suggestion {
        println!("Suggestion: {}", suggestion);
    }
}
```

### validate_content

```rust
pub fn validate_content(options: &ValidationOptions) -> Result<Vec<ValidationResult>>
```

Main function to validate content files based on provided options.

#### Parameters

- `options` - Validation options specifying what to validate and how

#### Returns

- `Result<Vec<ValidationResult>>` - List of validation results for each file

#### Implementation Details

This function:
1. Loads the application configuration
2. Finds content files to validate based on the options
3. Collects all internal resources for link validation
4. Initializes the spell checker if spelling validation is requested
5. For each content file:
   - Reads the file content
   - Performs requested validations (links, markdown, spelling)
   - Collects all issues found
6. Returns validation results for all files

The function supports validating specific articles, topics, or all content, and can perform different types of validation based on the options.

#### Example

```rust
let options = ValidationOptions {
    article: Some("my-post".to_string()),
    topic: Some("blog".to_string()),
    validation_types: vec![ValidationType::Links, ValidationType::Spelling],
    check_external_links: true,
    external_link_timeout: 10,
    custom_dictionary: Some(PathBuf::from("dictionaries/technical.txt")),
    include_drafts: false,
};

let results = validate_content(&options)?;
for result in results {
    println!("Validation results for {}", result.file_path.display());
    for issue in result.issues {
        println!("- {}", issue.description);
    }
}
``` 