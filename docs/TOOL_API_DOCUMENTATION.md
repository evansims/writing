# Tool-Specific API Documentation

This document provides detailed documentation for the public APIs of each tool in the codebase. It covers the purpose, usage patterns, and extension points for each tool.

## Table of Contents

1. [Content Management Tools](#content-management-tools)
2. [Topic Management Tools](#topic-management-tools)
3. [Image Processing Tools](#image-processing-tools)
4. [Build and Deployment Tools](#build-and-deployment-tools)
5. [Utility Tools](#utility-tools)

## Content Management Tools

### content-new

The `content-new` tool provides APIs for creating new content items with appropriate metadata and structure.

#### Public API

```rust
/// Creates a new content item with the specified parameters
pub fn create_content(
    content_type: ContentType,
    slug: &str,
    title: &str,
    options: ContentCreationOptions,
) -> Result<ContentItem, ContentError> {
    // Implementation details
}

/// Options for content creation
pub struct ContentCreationOptions {
    /// The template to use for content creation
    pub template: Option<String>,

    /// Topics to associate with the content
    pub topics: Vec<String>,

    /// Whether to open the editor after creation
    pub open_editor: bool,

    /// Additional frontmatter fields
    pub frontmatter: HashMap<String, Value>,
}
```

#### Usage Example

```rust
let options = ContentCreationOptions {
    template: Some("article".to_string()),
    topics: vec!["rust".to_string(), "documentation".to_string()],
    open_editor: true,
    frontmatter: HashMap::new(),
};

let content = create_content(
    ContentType::Article,
    "my-first-article",
    "My First Article",
    options,
)?;
```

### content-edit

The `content-edit` tool provides APIs for modifying existing content.

#### Public API

```rust
/// Edits an existing content item
pub fn edit_content(
    content_path: &Path,
    edit_options: ContentEditOptions,
) -> Result<ContentItem, ContentError> {
    // Implementation details
}

/// Options for content editing
pub struct ContentEditOptions {
    /// Updates to frontmatter fields
    pub frontmatter_updates: HashMap<String, Value>,

    /// Whether to open the editor after applying updates
    pub open_editor: bool,

    /// Topics to add
    pub add_topics: Vec<String>,

    /// Topics to remove
    pub remove_topics: Vec<String>,
}
```

### content-delete

The `content-delete` tool provides APIs for removing content items.

#### Public API

```rust
/// Deletes a content item
pub fn delete_content(
    content_path: &Path,
    options: ContentDeletionOptions,
) -> Result<(), ContentError> {
    // Implementation details
}

/// Options for content deletion
pub struct ContentDeletionOptions {
    /// Whether to back up the deleted content
    pub backup: bool,

    /// Whether to force deletion without confirmation
    pub force: bool,
}
```

## Topic Management Tools

### topic-add

The `topic-add` tool provides APIs for creating and managing content topics.

#### Public API

```rust
/// Creates a new topic
pub fn create_topic(
    name: &str,
    description: Option<&str>,
    options: TopicCreationOptions,
) -> Result<Topic, TopicError> {
    // Implementation details
}

/// Options for topic creation
pub struct TopicCreationOptions {
    /// Parent topic for hierarchical organization
    pub parent: Option<String>,

    /// Whether the topic should be featured
    pub featured: bool,

    /// Additional metadata
    pub metadata: HashMap<String, Value>,
}
```

### topic-edit

The `topic-edit` tool provides APIs for modifying existing topics.

#### Public API

```rust
/// Edits an existing topic
pub fn edit_topic(
    name: &str,
    edit_options: TopicEditOptions,
) -> Result<Topic, TopicError> {
    // Implementation details
}

/// Options for topic editing
pub struct TopicEditOptions {
    /// New name for the topic
    pub new_name: Option<String>,

    /// New description for the topic
    pub new_description: Option<String>,

    /// New parent topic
    pub new_parent: Option<String>,

    /// Whether to change the featured status
    pub featured: Option<bool>,

    /// Updates to metadata fields
    pub metadata_updates: HashMap<String, Value>,
}
```

## Image Processing Tools

### image-optimize

The `image-optimize` tool provides APIs for processing and optimizing images for web use.

#### Public API

```rust
/// Optimizes an image for web use
pub fn optimize_image(
    source_path: &Path,
    options: ImageOptimizationOptions,
) -> Result<OptimizedImage, ImageError> {
    // Implementation details
}

/// Options for image optimization
pub struct ImageOptimizationOptions {
    /// Target formats to generate
    pub formats: Vec<ImageFormat>,

    /// Resize dimensions
    pub dimensions: Vec<ImageDimension>,

    /// Quality settings per format
    pub quality: HashMap<ImageFormat, u8>,

    /// Whether to strip metadata
    pub strip_metadata: bool,
}

/// Image format enum
pub enum ImageFormat {
    WebP,
    Avif,
    Jpeg,
    Png,
}

/// Image dimension struct
pub struct ImageDimension {
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub name: String,
}
```

### image-build

The `image-build` tool provides APIs for batch processing images.

#### Public API

```rust
/// Processes all images in a directory
pub fn batch_process_images(
    directory: &Path,
    options: BatchProcessingOptions,
) -> Result<BatchProcessingReport, ImageError> {
    // Implementation details
}

/// Options for batch image processing
pub struct BatchProcessingOptions {
    /// Default optimization options
    pub default_options: ImageOptimizationOptions,

    /// Whether to process recursively
    pub recursive: bool,

    /// Whether to process images already optimized
    pub force_reprocess: bool,

    /// Max number of concurrent processing tasks
    pub concurrency: usize,
}
```

## Build and Deployment Tools

### content-build

The `content-build` tool provides APIs for generating static content.

#### Public API

```rust
/// Builds static content
pub fn build_content(
    options: BuildOptions,
) -> Result<BuildReport, BuildError> {
    // Implementation details
}

/// Options for content building
pub struct BuildOptions {
    /// Output directory
    pub output_dir: PathBuf,

    /// Whether to use incremental building
    pub incremental: bool,

    /// Content types to include
    pub content_types: Option<Vec<ContentType>>,

    /// Optimization level
    pub optimization_level: OptimizationLevel,
}

/// Optimization level enum
pub enum OptimizationLevel {
    None,
    Basic,
    Full,
}
```

### llms-generate

The `llms-generate` tool provides APIs for generating LLM-friendly content.

#### Public API

```rust
/// Generates LLM-friendly content
pub fn generate_llms_content(
    options: LlmsGenerationOptions,
) -> Result<LlmsGenerationReport, LlmsError> {
    // Implementation details
}

/// Options for LLMs content generation
pub struct LlmsGenerationOptions {
    /// Output directory
    pub output_dir: PathBuf,

    /// Whether to generate both full and summary files
    pub generate_full: bool,

    /// Content types to include
    pub content_types: Option<Vec<ContentType>>,
}
```

## Utility Tools

### toc-generate

The `toc-generate` tool provides APIs for generating table of contents.

#### Public API

```rust
/// Generates a table of contents for a markdown file
pub fn generate_toc(
    file_path: &Path,
    options: TocOptions,
) -> Result<String, TocError> {
    // Implementation details
}

/// Options for TOC generation
pub struct TocOptions {
    /// Maximum heading depth to include
    pub max_depth: usize,

    /// Whether to add links to headings
    pub add_links: bool,

    /// Whether to add line numbers
    pub add_line_numbers: bool,

    /// Optional title for the TOC
    pub title: Option<String>,
}
```

### content-validate

The `content-validate` tool provides APIs for validating content structure and metadata.

#### Public API

```rust
/// Validates content items
pub fn validate_content(
    paths: &[PathBuf],
    options: ValidationOptions,
) -> Result<ValidationReport, ValidationError> {
    // Implementation details
}

/// Options for content validation
pub struct ValidationOptions {
    /// Validation ruleset to apply
    pub ruleset: ValidationRuleset,

    /// Whether to fix common issues automatically
    pub auto_fix: bool,

    /// Whether to check links
    pub check_links: bool,

    /// Whether to validate images
    pub validate_images: bool,
}

/// Validation ruleset enum
pub enum ValidationRuleset {
    Basic,
    Standard,
    Strict,
}
```

### content-stats

The `content-stats` tool provides APIs for generating content statistics.

#### Public API

```rust
/// Generates statistics for content
pub fn generate_stats(
    options: StatsOptions,
) -> Result<StatsReport, StatsError> {
    // Implementation details
}

/// Options for statistics generation
pub struct StatsOptions {
    /// Content types to include
    pub content_types: Option<Vec<ContentType>>,

    /// Whether to include reading time estimates
    pub include_reading_time: bool,

    /// Whether to include word counts
    pub include_word_counts: bool,

    /// Whether to include topic distribution
    pub include_topic_distribution: bool,
}
```

## Extension Points

Each tool provides extension points through trait implementations and plugin hooks. For detailed information on extending these tools, see the [Extension Development Guide](./EXTENSION_DEVELOPMENT_GUIDE.md) and [Plugin System Architecture](./PLUGIN_SYSTEM_ARCHITECTURE.md).
