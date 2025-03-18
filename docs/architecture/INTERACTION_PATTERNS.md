# Interaction Patterns

This document describes the key interaction patterns used throughout the system, providing guidance on how components communicate and collaborate.

## Overview

The codebase uses several consistent interaction patterns to maintain modularity, testability, and flexibility. Understanding these patterns is crucial for maintaining and extending the codebase.

Key interaction patterns include:

1. **Command Pattern**: Used for encapsulating requests as objects
2. **Factory Pattern**: Used for creating objects without specifying concrete classes
3. **Observer Pattern**: Used for notifications and event handling
4. **Strategy Pattern**: Used for swappable algorithms
5. **Dependency Injection**: Used for providing dependencies to components
6. **Trait-based Polymorphism**: Used for defining abstractions and behaviors

## Command Pattern

The command pattern is used extensively in the CLI tools to encapsulate requests as objects.

### Implementation

```rust
// Command trait definition
pub trait Command {
    fn execute(&self) -> Result<(), Error>;
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;
}

// Concrete command implementation
pub struct NewContentCommand {
    content_type: ContentType,
    slug: String,
    title: String,
    options: ContentCreationOptions,
}

impl Command for NewContentCommand {
    fn execute(&self) -> Result<(), Error> {
        // Implementation details
    }

    fn name(&self) -> &'static str {
        "new-content"
    }

    fn description(&self) -> &'static str {
        "Creates a new content item"
    }
}

// Command executor
pub struct CommandExecutor {
    commands: HashMap<String, Box<dyn Command>>,
}

impl CommandExecutor {
    pub fn execute(&self, name: &str) -> Result<(), Error> {
        match self.commands.get(name) {
            Some(command) => command.execute(),
            None => Err(Error::UnknownCommand(name.to_string())),
        }
    }
}
```

### Usage

Commands are typically created and executed by the CLI parser:

```rust
let command = NewContentCommand::new(
    ContentType::Article,
    "my-article".to_string(),
    "My Article".to_string(),
    options,
);

let executor = CommandExecutor::new();
executor.register("new-article", Box::new(command));
executor.execute("new-article")?;
```

### Benefits

- Decouples the requester from the performer
- Enables queueing, logging, and undo operations
- Simplifies adding new commands without modifying existing code
- Facilitates testing through command mocking

## Factory Pattern

The factory pattern is used to create objects without specifying concrete classes, allowing for flexibility in object creation.

### Implementation

```rust
// Factory trait
pub trait ContentFactory {
    fn create(&self, content_type: ContentType, options: &ContentCreationOptions) -> Result<Box<dyn Content>, Error>;
}

// Concrete factory implementation
pub struct DefaultContentFactory;

impl ContentFactory for DefaultContentFactory {
    fn create(&self, content_type: ContentType, options: &ContentCreationOptions) -> Result<Box<dyn Content>, Error> {
        match content_type {
            ContentType::Article => Ok(Box::new(Article::new(options)?)),
            ContentType::Page => Ok(Box::new(Page::new(options)?)),
            ContentType::Note => Ok(Box::new(Note::new(options)?)),
            // ...
        }
    }
}
```

### Usage

```rust
let factory = DefaultContentFactory;
let content = factory.create(ContentType::Article, &options)?;
content.save()?;
```

### Benefits

- Hides concrete implementation details
- Centralizes object creation logic
- Simplifies adding new types without modifying client code
- Facilitates testing through factory mocking

## Observer Pattern

The observer pattern is used for notifications and event handling, particularly in long-running processes like builds.

### Implementation

```rust
// Event enum
pub enum BuildEvent {
    Started,
    FileProcessed(PathBuf),
    Error(Error),
    Completed,
}

// Observer trait
pub trait BuildObserver {
    fn on_event(&self, event: &BuildEvent);
}

// Subject implementation
pub struct BuildProcess {
    observers: Vec<Box<dyn BuildObserver>>,
}

impl BuildProcess {
    pub fn add_observer(&mut self, observer: Box<dyn BuildObserver>) {
        self.observers.push(observer);
    }

    pub fn notify(&self, event: BuildEvent) {
        for observer in &self.observers {
            observer.on_event(&event);
        }
    }

    pub fn build(&self, options: BuildOptions) -> Result<(), Error> {
        self.notify(BuildEvent::Started);

        // Build implementation
        // ...

        self.notify(BuildEvent::Completed);
        Ok(())
    }
}
```

### Usage

```rust
let mut build_process = BuildProcess::new();

// Add console logger observer
build_process.add_observer(Box::new(ConsoleLogger::new()));

// Add progress bar observer
build_process.add_observer(Box::new(ProgressBar::new()));

// Run the build
build_process.build(options)?;
```

### Benefits

- Decouples subjects from observers
- Enables multiple observers without modifying the subject
- Supports broadcasting notifications to multiple interested parties
- Facilitates extension through new observers

## Strategy Pattern

The strategy pattern is used to define a family of interchangeable algorithms that can be selected at runtime.

### Implementation

```rust
// Strategy trait
pub trait ImageResizeStrategy {
    fn resize(&self, image: &Image, width: u32, height: u32) -> Result<Image, Error>;
}

// Concrete strategies
pub struct LanczosResizer;
impl ImageResizeStrategy for LanczosResizer {
    fn resize(&self, image: &Image, width: u32, height: u32) -> Result<Image, Error> {
        // Lanczos resizing implementation
    }
}

pub struct NearestResizer;
impl ImageResizeStrategy for NearestResizer {
    fn resize(&self, image: &Image, width: u32, height: u32) -> Result<Image, Error> {
        // Nearest neighbor resizing implementation
    }
}

// Context using the strategy
pub struct ImageProcessor {
    resize_strategy: Box<dyn ImageResizeStrategy>,
}

impl ImageProcessor {
    pub fn new(resize_strategy: Box<dyn ImageResizeStrategy>) -> Self {
        Self { resize_strategy }
    }

    pub fn resize(&self, image: &Image, width: u32, height: u32) -> Result<Image, Error> {
        self.resize_strategy.resize(image, width, height)
    }
}
```

### Usage

```rust
// Create with Lanczos resizing (high quality but slower)
let processor = ImageProcessor::new(Box::new(LanczosResizer));

// For batch processing where speed matters more than quality
let fast_processor = ImageProcessor::new(Box::new(NearestResizer));
```

### Benefits

- Enables runtime selection of algorithms
- Isolates algorithm implementation from clients
- Facilitates adding new strategies without modifying clients
- Supports testing through strategy mocking

## Dependency Injection

Dependency injection is used throughout the codebase to provide dependencies to components, enhancing testability and flexibility.

### Implementation

```rust
// Component with injected dependencies
pub struct ContentValidator {
    schema_provider: Box<dyn SchemaProvider>,
    link_checker: Box<dyn LinkChecker>,
}

impl ContentValidator {
    pub fn new(
        schema_provider: Box<dyn SchemaProvider>,
        link_checker: Box<dyn LinkChecker>,
    ) -> Self {
        Self {
            schema_provider,
            link_checker,
        }
    }

    pub fn validate(&self, content: &Content) -> Result<ValidationReport, Error> {
        // Use injected dependencies for validation
        let schema = self.schema_provider.get_schema(content.content_type())?;
        let validation_result = schema.validate(content)?;

        if content.has_links() {
            let link_results = self.link_checker.check_links(content.links())?;
            // Combine results
        }

        // Return combined report
    }
}
```

### Usage

```rust
// Production setup
let validator = ContentValidator::new(
    Box::new(FileSystemSchemaProvider::new()),
    Box::new(HttpLinkChecker::new()),
);

// Test setup
let validator = ContentValidator::new(
    Box::new(MockSchemaProvider::new()),
    Box::new(MockLinkChecker::new()),
);
```

### Benefits

- Decouples component implementation from dependency creation
- Enhances testability through dependency mocking
- Supports different implementations for different environments
- Enables reusing components with different dependencies

## Trait-based Polymorphism

Rust's trait system is used extensively for defining abstractions and behaviors.

### Implementation

```rust
// Content trait defining behavior
pub trait Content {
    fn content_type(&self) -> ContentType;
    fn title(&self) -> &str;
    fn body(&self) -> &str;
    fn frontmatter(&self) -> &HashMap<String, Value>;
    fn save(&self) -> Result<(), Error>;
    fn validate(&self) -> Result<ValidationReport, Error>;
}

// Implementation for Article
pub struct Article {
    title: String,
    body: String,
    frontmatter: HashMap<String, Value>,
}

impl Content for Article {
    fn content_type(&self) -> ContentType {
        ContentType::Article
    }

    fn title(&self) -> &str {
        &self.title
    }

    fn body(&self) -> &str {
        &self.body
    }

    fn frontmatter(&self) -> &HashMap<String, Value> {
        &self.frontmatter
    }

    fn save(&self) -> Result<(), Error> {
        // Implementation details
    }

    fn validate(&self) -> Result<ValidationReport, Error> {
        // Implementation details
    }
}
```

### Usage

```rust
fn process_content(content: &dyn Content) -> Result<(), Error> {
    println!("Processing {}: {}", content.content_type(), content.title());
    content.validate()?;
    content.save()?;
    Ok(())
}

// Can be used with any type that implements Content
let article = Article::new("My Article", "Article body", HashMap::new());
process_content(&article)?;
```

### Benefits

- Defines clear contracts for behavior
- Enables code reuse across different types
- Facilitates testing through trait mocking
- Provides compile-time guarantees of behavior

## Composite Interactions

Many components in the system combine multiple patterns for more complex interactions.

### Example: Content Build System

The content build system combines the Command, Observer, Strategy, and Dependency Injection patterns:

```rust
// Build command (Command Pattern)
pub struct BuildCommand {
    options: BuildOptions,
    observer_factory: Box<dyn BuildObserverFactory>,
    renderer_factory: Box<dyn RendererFactory>,
}

impl Command for BuildCommand {
    fn execute(&self) -> Result<(), Error> {
        // Create build process (Observer Pattern)
        let mut build_process = BuildProcess::new();

        // Add observers from factory (Dependency Injection)
        for observer in self.observer_factory.create_observers() {
            build_process.add_observer(observer);
        }

        // Create appropriate renderer (Strategy Pattern)
        let renderer = self.renderer_factory.create_renderer(&self.options)?;

        // Execute build with selected renderer
        build_process.build(&self.options, renderer)
    }
}
```

## Testing Interaction Patterns

Each pattern described above facilitates testing in different ways:

### Command Pattern Testing

```rust
#[test]
fn test_new_content_command() {
    let command = NewContentCommand::new(
        ContentType::Article,
        "test-article".to_string(),
        "Test Article".to_string(),
        ContentCreationOptions::default(),
    );

    // Use a test filesystem
    with_test_fs(|fs| {
        command.execute().unwrap();

        // Assert file was created
        assert!(fs.file_exists("content/articles/test-article.md"));

        // Assert content is correct
        let content = fs.read_file("content/articles/test-article.md").unwrap();
        assert!(content.contains("title: Test Article"));
    });
}
```

### Factory Pattern Testing

```rust
#[test]
fn test_content_factory() {
    let factory = DefaultContentFactory;

    let options = ContentCreationOptions {
        template: None,
        topics: vec!["test".to_string()],
        open_editor: false,
        frontmatter: HashMap::new(),
    };

    let article = factory.create(ContentType::Article, &options).unwrap();
    assert_eq!(article.content_type(), ContentType::Article);

    let page = factory.create(ContentType::Page, &options).unwrap();
    assert_eq!(page.content_type(), ContentType::Page);
}
```

### Observer Pattern Testing

```rust
#[test]
fn test_build_process_observers() {
    // Create a mock observer that records events
    let mock_observer = MockObserver::new();

    let mut build_process = BuildProcess::new();
    build_process.add_observer(Box::new(mock_observer.clone()));

    // Run a build
    build_process.build(BuildOptions::default()).unwrap();

    // Assert events were received in correct order
    let events = mock_observer.events();
    assert_eq!(events[0], BuildEvent::Started);
    assert_eq!(events[events.len() - 1], BuildEvent::Completed);

    // Assert specific events occurred
    assert!(events.iter().any(|e| matches!(e, BuildEvent::FileProcessed(_))));
}
```

## Conclusion

By understanding and consistently applying these interaction patterns, developers can maintain a clean, testable, and extensible codebase. These patterns form the foundation of component interactions throughout the system, providing a common language and approach for solving recurring design problems.

When extending the system, consider which patterns are most appropriate for the problem at hand, and aim to follow the established patterns for consistency and maintainability.
