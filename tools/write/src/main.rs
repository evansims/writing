use anyhow::Result;
use clap::{Parser, Subcommand};
use colored::*;
use console::Term;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use dialoguer::{theme::ColorfulTheme, Confirm, Input, Select};
use indicatif::{ProgressBar, ProgressStyle};
use std::{io, time::Duration};
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Terminal,
};

mod tools;
mod config;

#[derive(Parser)]
#[command(
    name = "write",
    author = "Evan Sims <evan@evansims.com>",
    version,
    about = "A comprehensive tool for managing writing content, topics, images, and output files",
    long_about = "The Content Management CLI tool provides a set of commands for managing writing content, topics, images, and build processes for your writing project. You can create, edit, move, and delete content; manage topics; optimize images; and build content into various formats.

When run without commands, it launches an interactive CLI experience for easier navigation through the tool's features."
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Content management commands
    #[command(subcommand)]
    Content(ContentCommands),
    
    /// Topic management commands
    #[command(subcommand)]
    Topic(TopicCommands),
    
    /// Image management commands
    #[command(subcommand)]
    Image(ImageCommands),
    
    /// Build commands
    #[command(subcommand)]
    Build(BuildCommands),
    
    /// Generate statistics about your content
    Stats {
        /// Content slug to generate statistics for
        #[arg(long, short)]
        slug: Option<String>,
        
        /// Topic to filter content by
        #[arg(long, short)]
        topic: Option<String>,
        
        /// Include draft content
        #[arg(long)]
        include_drafts: bool,
        
        /// How to sort the statistics (date, words, time)
        #[arg(long, value_name = "SORT_BY")]
        sort_by: String,
        
        /// Show detailed statistics
        #[arg(long)]
        detailed: bool,
    },
}

/// Commands for content management
#[derive(Subcommand)]
enum ContentCommands {
    /// Create new content
    #[command(about = "Create a new content item")]
    New {
        /// Title of the content
        #[arg(short, long)]
        title: Option<String>,
        
        /// Topic for the content
        #[arg(short, long)]
        topic: Option<String>,
        
        /// Tagline for the content
        #[arg(short, long)]
        tagline: Option<String>,
        
        /// Tags for the content (comma-separated)
        #[arg(short, long)]
        tags: Option<String>,
        
        /// Type of content (article, note, tutorial)
        #[arg(short, long, default_value = "article")]
        content_type: String,
        
        /// Create as draft
        #[arg(short, long)]
        draft: bool,
        
        /// Use template for content
        #[arg(short, long)]
        template: Option<String>,
        
        /// Introduction text for content
        #[arg(short, long)]
        introduction: Option<String>,
    },
    
    /// Template management
    #[command(subcommand, about = "Manage content templates")]
    Template(TemplateCommands),
    
    /// Edit existing content
    #[command(about = "Edit an existing content item")]
    Edit {
        /// Content slug to edit
        #[arg(long, help = "Slug of the content to edit")]
        slug: Option<String>,
        
        /// Topic of the content to edit
        #[arg(long, help = "Topic of the content to edit")]
        topic: Option<String>,
        
        /// Edit frontmatter only
        #[arg(long, help = "Edit only the frontmatter")]
        frontmatter: bool,
        
        /// Edit content body only
        #[arg(long, help = "Edit only the content body")]
        content: bool,
    },
    
    /// Move content (change slug and/or topic)
    #[command(about = "Move content by changing slug or topic")]
    Move {
        /// Current content slug
        #[arg(long, help = "Current slug of the content to move")]
        slug: Option<String>,
        
        /// New content slug
        #[arg(long, help = "New slug for the content")]
        new_slug: Option<String>,
        
        /// Current topic
        #[arg(long, help = "Current topic of the content")]
        topic: Option<String>,
        
        /// New topic
        #[arg(long, help = "New topic for the content")]
        new_topic: Option<String>,
    },
    
    /// Delete content
    #[command(about = "Delete existing content")]
    Delete {
        /// Slug of content to delete
        #[arg(long, help = "Slug of the content to delete")]
        slug: Option<String>,
        
        /// Topic of content to delete
        #[arg(long, help = "Topic of the content to delete")]
        topic: Option<String>,
        
        /// Skip confirmation
        #[arg(long, help = "Skip confirmation prompt")]
        force: bool,
    },
    
    /// List all content
    #[command(about = "List all content items")]
    List,
}

/// Commands for template management
#[derive(Subcommand)]
enum TemplateCommands {
    /// List all templates
    #[command(about = "List all available templates")]
    List,
    
    /// Create a new template
    #[command(about = "Create a new content template")]
    Create {
        /// Template name
        #[arg(long, help = "Name for the template")]
        name: Option<String>,
        
        /// Content type
        #[arg(long, help = "Content type for the template (article, note, etc.)")]
        content_type: Option<String>,
    },
}

/// Commands for topic management
#[derive(Subcommand)]
enum TopicCommands {
    /// List all topics
    #[command(about = "List all available topics")]
    List,
    
    /// Add a new topic
    #[command(about = "Add a new topic")]
    Add {
        /// Topic key (identifier)
        #[arg(long, help = "Unique key for the topic")]
        key: Option<String>,
        
        /// Topic name
        #[arg(long, help = "Display name for the topic")]
        name: Option<String>,
        
        /// Topic description
        #[arg(long, help = "Description of the topic")]
        description: Option<String>,
        
        /// Directory path
        #[arg(long, help = "Directory path for the topic")]
        path: Option<String>,
    },
    
    /// Edit an existing topic
    #[command(about = "Edit an existing topic")]
    Edit {
        /// Topic key to edit
        #[arg(long, help = "Key of the topic to edit")]
        key: Option<String>,
        
        /// New topic name
        #[arg(long, help = "New name for the topic")]
        name: Option<String>,
        
        /// New topic description
        #[arg(long, help = "New description for the topic")]
        description: Option<String>,
    },
    
    /// Rename a topic
    #[command(about = "Rename a topic and update its properties")]
    Rename {
        /// Current topic key
        #[arg(long, help = "Current key of the topic to rename")]
        key: Option<String>,
        
        /// New topic key
        #[arg(long, help = "New key for the topic")]
        new_key: Option<String>,
        
        /// New topic name
        #[arg(long, help = "New name for the topic")]
        new_name: Option<String>,
        
        /// New directory path
        #[arg(long, help = "New directory path for the topic")]
        new_path: Option<String>,
    },
    
    /// Delete a topic
    #[command(about = "Delete a topic")]
    Delete {
        /// Topic key to delete
        #[arg(long, help = "Key of the topic to delete")]
        key: Option<String>,
        
        /// Target topic for content migration
        #[arg(long, help = "Target topic for migrating content")]
        target: Option<String>,
        
        /// Skip confirmation
        #[arg(long, help = "Skip confirmation prompt")]
        force: bool,
    },
}

/// Commands for image management
#[derive(Subcommand)]
enum ImageCommands {
    /// Optimize an image
    #[command(about = "Optimize an image for web use")]
    Optimize {
        /// Source image path
        #[arg(long, help = "Path to the source image")]
        source: String,
        
        /// Article slug
        #[arg(long, help = "Slug of the article the image belongs to")]
        article: String,
        
        /// Topic key
        #[arg(long, help = "Key of the topic the article belongs to")]
        topic: Option<String>,
    },
    
    /// Build image variants
    #[command(about = "Build image variants for all content")]
    Build {
        /// Specific article to build images for
        #[arg(long, help = "Build images for a specific article only")]
        article: Option<String>,
        
        /// Specific topic to build images for
        #[arg(long, help = "Build images for a specific topic only")]
        topic: Option<String>,
        
        /// Skip confirmation
        #[arg(long, help = "Skip confirmation prompt")]
        force: bool,
    },
}

/// Commands for build and generation
#[derive(Subcommand)]
enum BuildCommands {
    /// Build content files
    #[command(about = "Build content into static files (JSON, HTML, RSS, sitemap)")]
    Content {
        /// Output directory
        #[arg(long, help = "Output directory for generated files")]
        output_dir: Option<String>,
        
        /// Specific content to build
        #[arg(long, help = "Build only a specific content item by slug")]
        slug: Option<String>,
        
        /// Specific topic to build
        #[arg(long, help = "Build only content for a specific topic")]
        topic: Option<String>,
        
        /// Include draft content
        #[arg(long, help = "Include draft content in the build")]
        include_drafts: bool,
        
        /// Skip HTML generation
        #[arg(long, help = "Skip HTML file generation")]
        skip_html: bool,
        
        /// Skip JSON generation
        #[arg(long, help = "Skip JSON file generation")]
        skip_json: bool,
        
        /// Skip RSS generation
        #[arg(long, help = "Skip RSS feed generation")]
        skip_rss: bool,
        
        /// Skip sitemap generation
        #[arg(long, help = "Skip sitemap generation")]
        skip_sitemap: bool,
        
        /// Show verbose output
        #[arg(long, help = "Show verbose output during build")]
        verbose: bool,
    },
    
    /// Generate table of contents
    #[command(about = "Generate a table of contents markdown file")]
    Toc {
        /// Output file path
        #[arg(long, help = "Path for the generated table of contents file")]
        output: Option<String>,
    },
    
    /// Generate LLMs files
    #[command(about = "Generate llms.txt and llms-full.txt files")]
    Llms {
        /// Site URL for absolute links
        #[arg(long, help = "Site URL for generating absolute links")]
        site_url: Option<String>,
        
        /// Output directory
        #[arg(long, help = "Output directory for generated files")]
        output_dir: Option<String>,
        
        /// Include draft content
        #[arg(long, help = "Include draft content in the files")]
        include_drafts: bool,
    },
}

enum MenuItem {
    Content,
    Topics,
    Images,
    Build,
    Stats,
    Help,
    Quit,
}

enum ContentMenuItem {
    New,
    Edit,
    Move,
    Delete,
    List,
    Templates,
    Back,
}

enum TopicsMenuItem {
    List,
    Add,
    Edit,
    Rename,
    Delete,
    Back,
}

enum ImagesMenuItem {
    Optimize,
    Build,
    Back,
}

enum BuildMenuItem {
    Content,
    Toc,
    Llms,
    Back,
}

struct App {
    menu_state: MenuState,
    selected_menu_item: MenuItem,
    selected_content_menu_item: ContentMenuItem,
    selected_topics_menu_item: TopicsMenuItem,
    selected_images_menu_item: ImagesMenuItem,
    selected_build_menu_item: BuildMenuItem,
}

enum MenuState {
    Main,
    Content,
    Topics,
    Images,
    Build,
}

impl App {
    fn new() -> Self {
        Self {
            menu_state: MenuState::Main,
            selected_menu_item: MenuItem::Content,
            selected_content_menu_item: ContentMenuItem::New,
            selected_topics_menu_item: TopicsMenuItem::List,
            selected_images_menu_item: ImagesMenuItem::Optimize,
            selected_build_menu_item: BuildMenuItem::Content,
        }
    }

    fn handle_key_event(&mut self, key: KeyCode) -> bool {
        match self.menu_state {
            MenuState::Main => self.handle_main_menu_key(key),
            MenuState::Content => self.handle_content_menu_key(key),
            MenuState::Topics => self.handle_topics_menu_key(key),
            MenuState::Images => self.handle_images_menu_key(key),
            MenuState::Build => self.handle_build_menu_key(key),
        }
    }

    fn handle_main_menu_key(&mut self, key: KeyCode) -> bool {
        match key {
            KeyCode::Up => {
                self.selected_menu_item = match self.selected_menu_item {
                    MenuItem::Content => MenuItem::Quit,
                    MenuItem::Topics => MenuItem::Content,
                    MenuItem::Images => MenuItem::Topics,
                    MenuItem::Build => MenuItem::Images,
                    MenuItem::Stats => MenuItem::Build,
                    MenuItem::Help => MenuItem::Stats,
                    MenuItem::Quit => MenuItem::Help,
                };
                false
            }
            KeyCode::Down => {
                self.selected_menu_item = match self.selected_menu_item {
                    MenuItem::Content => MenuItem::Topics,
                    MenuItem::Topics => MenuItem::Images,
                    MenuItem::Images => MenuItem::Build,
                    MenuItem::Build => MenuItem::Stats,
                    MenuItem::Stats => MenuItem::Help,
                    MenuItem::Help => MenuItem::Quit,
                    MenuItem::Quit => MenuItem::Content,
                };
                false
            }
            KeyCode::Enter => {
                match self.selected_menu_item {
                    MenuItem::Content => {
                        self.menu_state = MenuState::Content;
                        false
                    }
                    MenuItem::Topics => {
                        self.menu_state = MenuState::Topics;
                        false
                    }
                    MenuItem::Images => {
                        self.menu_state = MenuState::Images;
                        false
                    }
                    MenuItem::Build => {
                        self.menu_state = MenuState::Build;
                        false
                    }
                    MenuItem::Stats => {
                        // Handle stats directly
                        false
                    }
                    MenuItem::Help => {
                        let _ = show_help();
                        false
                    }
                    MenuItem::Quit => true,
                }
            }
            KeyCode::Char('q') => true,
            _ => false,
        }
    }

    fn handle_content_menu_key(&mut self, key: KeyCode) -> bool {
        match key {
            KeyCode::Up => {
                self.selected_content_menu_item = match self.selected_content_menu_item {
                    ContentMenuItem::New => ContentMenuItem::Back,
                    ContentMenuItem::Edit => ContentMenuItem::New,
                    ContentMenuItem::Move => ContentMenuItem::Edit,
                    ContentMenuItem::Delete => ContentMenuItem::Move,
                    ContentMenuItem::List => ContentMenuItem::Delete,
                    ContentMenuItem::Templates => ContentMenuItem::List,
                    ContentMenuItem::Back => ContentMenuItem::List,
                };
                false
            }
            KeyCode::Down => {
                self.selected_content_menu_item = match self.selected_content_menu_item {
                    ContentMenuItem::New => ContentMenuItem::Edit,
                    ContentMenuItem::Edit => ContentMenuItem::Move,
                    ContentMenuItem::Move => ContentMenuItem::Delete,
                    ContentMenuItem::Delete => ContentMenuItem::List,
                    ContentMenuItem::List => ContentMenuItem::Templates,
                    ContentMenuItem::Templates => ContentMenuItem::Back,
                    ContentMenuItem::Back => ContentMenuItem::New,
                };
                false
            }
            KeyCode::Enter => {
                match self.selected_content_menu_item {
                    ContentMenuItem::Back => {
                        self.menu_state = MenuState::Main;
                        false
                    }
                    _ => {
                        // Handle content actions
                        true
                    }
                }
            }
            KeyCode::Esc => {
                self.menu_state = MenuState::Main;
                false
            }
            KeyCode::Char('q') => true,
            _ => false,
        }
    }

    fn handle_topics_menu_key(&mut self, key: KeyCode) -> bool {
        match key {
            KeyCode::Up => {
                self.selected_topics_menu_item = match self.selected_topics_menu_item {
                    TopicsMenuItem::List => TopicsMenuItem::Back,
                    TopicsMenuItem::Add => TopicsMenuItem::List,
                    TopicsMenuItem::Edit => TopicsMenuItem::Add,
                    TopicsMenuItem::Rename => TopicsMenuItem::Edit,
                    TopicsMenuItem::Delete => TopicsMenuItem::Rename,
                    TopicsMenuItem::Back => TopicsMenuItem::Delete,
                };
                false
            }
            KeyCode::Down => {
                self.selected_topics_menu_item = match self.selected_topics_menu_item {
                    TopicsMenuItem::List => TopicsMenuItem::Add,
                    TopicsMenuItem::Add => TopicsMenuItem::Edit,
                    TopicsMenuItem::Edit => TopicsMenuItem::Rename,
                    TopicsMenuItem::Rename => TopicsMenuItem::Delete,
                    TopicsMenuItem::Delete => TopicsMenuItem::Back,
                    TopicsMenuItem::Back => TopicsMenuItem::List,
                };
                false
            }
            KeyCode::Enter => {
                match self.selected_topics_menu_item {
                    TopicsMenuItem::Back => {
                        self.menu_state = MenuState::Main;
                        false
                    }
                    _ => {
                        // Handle topics actions
                        true
                    }
                }
            }
            KeyCode::Esc => {
                self.menu_state = MenuState::Main;
                false
            }
            KeyCode::Char('q') => true,
            _ => false,
        }
    }

    fn handle_images_menu_key(&mut self, key: KeyCode) -> bool {
        match key {
            KeyCode::Up => {
                self.selected_images_menu_item = match self.selected_images_menu_item {
                    ImagesMenuItem::Optimize => ImagesMenuItem::Back,
                    ImagesMenuItem::Build => ImagesMenuItem::Optimize,
                    ImagesMenuItem::Back => ImagesMenuItem::Build,
                };
                false
            }
            KeyCode::Down => {
                self.selected_images_menu_item = match self.selected_images_menu_item {
                    ImagesMenuItem::Optimize => ImagesMenuItem::Build,
                    ImagesMenuItem::Build => ImagesMenuItem::Back,
                    ImagesMenuItem::Back => ImagesMenuItem::Optimize,
                };
                false
            }
            KeyCode::Enter => {
                match self.selected_images_menu_item {
                    ImagesMenuItem::Back => {
                        self.menu_state = MenuState::Main;
                        false
                    }
                    _ => {
                        // Handle images actions
                        true
                    }
                }
            }
            KeyCode::Esc => {
                self.menu_state = MenuState::Main;
                false
            }
            KeyCode::Char('q') => true,
            _ => false,
        }
    }

    fn handle_build_menu_key(&mut self, key: KeyCode) -> bool {
        match key {
            KeyCode::Up => {
                self.selected_build_menu_item = match self.selected_build_menu_item {
                    BuildMenuItem::Content => BuildMenuItem::Back,
                    BuildMenuItem::Toc => BuildMenuItem::Content,
                    BuildMenuItem::Llms => BuildMenuItem::Toc,
                    BuildMenuItem::Back => BuildMenuItem::Llms,
                };
                false
            }
            KeyCode::Down => {
                self.selected_build_menu_item = match self.selected_build_menu_item {
                    BuildMenuItem::Content => BuildMenuItem::Toc,
                    BuildMenuItem::Toc => BuildMenuItem::Llms,
                    BuildMenuItem::Llms => BuildMenuItem::Back,
                    BuildMenuItem::Back => BuildMenuItem::Content,
                };
                false
            }
            KeyCode::Enter => {
                match self.selected_build_menu_item {
                    BuildMenuItem::Back => {
                        self.menu_state = MenuState::Main;
                        false
                    }
                    _ => {
                        // Handle build actions
                        true
                    }
                }
            }
            KeyCode::Esc => {
                self.menu_state = MenuState::Main;
                false
            }
            KeyCode::Char('q') => true,
            _ => false,
        }
    }
}

fn ui<B: tui::backend::Backend>(f: &mut tui::Frame<B>, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(f.size());

    let title = Paragraph::new("Writing Management System")
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);

    match app.menu_state {
        MenuState::Main => render_main_menu(f, app, chunks[1]),
        MenuState::Content => render_content_menu(f, app, chunks[1]),
        MenuState::Topics => render_topics_menu(f, app, chunks[1]),
        MenuState::Images => render_images_menu(f, app, chunks[1]),
        MenuState::Build => render_build_menu(f, app, chunks[1]),
    }
}

fn render_main_menu<B: tui::backend::Backend>(f: &mut tui::Frame<B>, app: &App, area: tui::layout::Rect) {
    let items = vec![
        ListItem::new("Content Management"),
        ListItem::new("Topics Management"),
        ListItem::new("Images Management"),
        ListItem::new("Build Operations"),
        ListItem::new("Content Statistics"),
        ListItem::new("Help"),
        ListItem::new("Quit"),
    ];

    let selected_index = match app.selected_menu_item {
        MenuItem::Content => 0,
        MenuItem::Topics => 1,
        MenuItem::Images => 2,
        MenuItem::Build => 3,
        MenuItem::Stats => 4,
        MenuItem::Help => 5,
        MenuItem::Quit => 6,
    };

    let list = List::new(items)
        .block(Block::default().title("Main Menu").borders(Borders::ALL))
        .highlight_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        .highlight_symbol(">> ");

    let mut state = tui::widgets::ListState::default();
    state.select(Some(selected_index));
    f.render_stateful_widget(list, area, &mut state);
}

fn render_content_menu<B: tui::backend::Backend>(f: &mut tui::Frame<B>, app: &App, area: tui::layout::Rect) {
    let items = vec![
        ListItem::new("Create New Content"),
        ListItem::new("Edit Existing Content"),
        ListItem::new("Move Content"),
        ListItem::new("Delete Content"),
        ListItem::new("List All Content"),
        ListItem::new("Manage Templates"),
        ListItem::new("Back to Main Menu"),
    ];

    let selected_index = match app.selected_content_menu_item {
        ContentMenuItem::New => 0,
        ContentMenuItem::Edit => 1,
        ContentMenuItem::Move => 2,
        ContentMenuItem::Delete => 3,
        ContentMenuItem::List => 4,
        ContentMenuItem::Templates => 5,
        ContentMenuItem::Back => 6,
    };

    let list = List::new(items)
        .block(Block::default().title("Content Management").borders(Borders::ALL))
        .highlight_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        .highlight_symbol(">> ");

    let mut state = tui::widgets::ListState::default();
    state.select(Some(selected_index));
    f.render_stateful_widget(list, area, &mut state);
}

fn render_topics_menu<B: tui::backend::Backend>(f: &mut tui::Frame<B>, app: &App, area: tui::layout::Rect) {
    let items = vec![
        ListItem::new("List All Topics"),
        ListItem::new("Add New Topic"),
        ListItem::new("Edit Topic"),
        ListItem::new("Rename Topic"),
        ListItem::new("Delete Topic"),
        ListItem::new("Back to Main Menu"),
    ];

    let selected_index = match app.selected_topics_menu_item {
        TopicsMenuItem::List => 0,
        TopicsMenuItem::Add => 1,
        TopicsMenuItem::Edit => 2,
        TopicsMenuItem::Rename => 3,
        TopicsMenuItem::Delete => 4,
        TopicsMenuItem::Back => 5,
    };

    let list = List::new(items)
        .block(Block::default().title("Topics Management").borders(Borders::ALL))
        .highlight_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        .highlight_symbol(">> ");

    let mut state = tui::widgets::ListState::default();
    state.select(Some(selected_index));
    f.render_stateful_widget(list, area, &mut state);
}

fn render_images_menu<B: tui::backend::Backend>(f: &mut tui::Frame<B>, app: &App, area: tui::layout::Rect) {
    let items = vec![
        ListItem::new("Optimize Source Image"),
        ListItem::new("Build Optimized Images"),
        ListItem::new("Back to Main Menu"),
    ];

    let selected_index = match app.selected_images_menu_item {
        ImagesMenuItem::Optimize => 0,
        ImagesMenuItem::Build => 1,
        ImagesMenuItem::Back => 2,
    };

    let list = List::new(items)
        .block(Block::default().title("Images Management").borders(Borders::ALL))
        .highlight_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        .highlight_symbol(">> ");

    let mut state = tui::widgets::ListState::default();
    state.select(Some(selected_index));
    f.render_stateful_widget(list, area, &mut state);
}

fn render_build_menu<B: tui::backend::Backend>(f: &mut tui::Frame<B>, app: &App, area: tui::layout::Rect) {
    let items = vec![
        ListItem::new("Build Content"),
        ListItem::new("Generate Table of Contents"),
        ListItem::new("Generate LLMs Files"),
        ListItem::new("Back to Main Menu"),
    ];

    let selected_index = match app.selected_build_menu_item {
        BuildMenuItem::Content => 0,
        BuildMenuItem::Toc => 1,
        BuildMenuItem::Llms => 2,
        BuildMenuItem::Back => 3,
    };

    let list = List::new(items)
        .block(Block::default().title("Build Operations").borders(Borders::ALL))
        .highlight_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        .highlight_symbol(">> ");

    let mut state = tui::widgets::ListState::default();
    state.select(Some(selected_index));
    f.render_stateful_widget(list, area, &mut state);
}

fn run_interactive_tui() -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app state
    let mut app = App::new();

    loop {
        // Draw UI
        terminal.draw(|f| ui(f, &app))?;

        // Handle input
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if app.handle_key_event(key.code) {
                    break;
                }
            }
        }
    }

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

fn run_interactive_cli() -> Result<()> {
    let term = Term::stdout();
    term.clear_screen()?;

    println!("{}", "Welcome to the Content Management Tool".cyan().bold());
    println!("════════════════════════════════════\n");
    println!("This interactive tool helps you manage writing content, topics, images, and build outputs.\n");

    loop {
        let options = vec![
            "Content Management",
            "Topic Management",
            "Image Management",
            "Build & Generate",
            "Content Statistics",
            "Help",
            "Quit",
        ];

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select an option")
            .default(0)
            .items(&options)
            .interact_on_opt(&term)?;

        match selection {
            Some(0) => content_menu()?,
            Some(1) => topics_menu()?,
            Some(2) => images_menu()?,
            Some(3) => build_menu()?,
            Some(4) => stats_menu()?,
            Some(5) => show_help()?,
            Some(6) => break,
            _ => break,
        }

        term.clear_screen()?;
        println!("{}", "Welcome to the Content Management Tool".cyan().bold());
        println!("════════════════════════════════════\n");
    }

    println!("\nThank you for using the Content Management Tool!");
    Ok(())
}

fn show_help() -> Result<()> {
    println!("\n{}", "Help & Documentation".cyan().bold());
    println!("═══════════════════════\n");

    println!("{}", "The Content Management Tool provides a comprehensive set of features:".yellow());
    
    println!("\n{}", "Content Management:".bold());
    println!("  • Create new content with frontmatter and markdown");
    println!("  • Edit existing content, including frontmatter and content");
    println!("  • Move content between topics or rename slugs");
    println!("  • Delete content with optional confirmation");
    println!("  • List all content with filtering options");

    println!("\n{}", "Topic Management:".bold());
    println!("  • List all topics");
    println!("  • Add new topics with name, key, and description");
    println!("  • Edit topic metadata");
    println!("  • Rename topic keys and update paths");
    println!("  • Delete topics with optional content migration");

    println!("\n{}", "Image Management:".bold());
    println!("  • Optimize images for web use");
    println!("  • Build image variants (coming soon)");

    println!("\n{}", "Build & Generate:".bold());
    println!("  • Build content to HTML, JSON, and other formats");
    println!("  • Generate table of contents files");
    println!("  • Generate LLMs compatibility files");
    println!("  • Control build options like drafts, output formats, etc.");

    println!("\n{}", "Content Statistics:".bold());
    println!("  • Generate word counts and reading time statistics");
    println!("  • Filter by topic, content, or drafts");
    println!("  • Sort by various metrics");

    println!("\n{}", "For detailed help on any specific command, use the command line with --help:".italic());
    println!("  write content create --help");
    
    println!("\nPress Enter to return to the main menu...");
    Term::stdout().read_line()?;
    
    Ok(())
}

/// Handle content management commands
fn handle_content_command(cmd: &ContentCommands) -> Result<()> {
    match cmd {
        ContentCommands::New {
            title,
            topic,
            tagline,
            tags,
            content_type,
            draft,
            template,
            introduction,
        } => {
            println!("Creating new content with title: {:?}", title);
            tools::create_content(
                title.clone(),
                topic.clone(),
                tagline.clone(),
                tags.clone(),
                Some(content_type.clone()),
                *draft,
                template.clone(),
                introduction.clone()
            )?;
        },
        ContentCommands::Edit {
            slug,
            topic,
            frontmatter,
            content,
        } => {
            println!("Editing content with slug: {:?}", slug);
            tools::edit_content(
                slug.clone(),
                topic.clone(),
                *frontmatter,
                *content
            )?;
        },
        ContentCommands::Move {
            slug,
            new_slug,
            topic,
            new_topic,
        } => {
            println!("Moving content with slug: {:?}", slug);
            tools::move_content(
                slug.clone(),
                new_slug.clone(),
                topic.clone(),
                new_topic.clone()
            )?;
        },
        ContentCommands::Delete {
            slug,
            topic,
            force,
        } => {
            println!("Deleting content with slug: {:?}", slug);
            tools::delete_content(
                slug.clone(),
                topic.clone(),
                *force
            )?;
        },
        ContentCommands::List => {
            println!("Listing all content...");
            tools::list_content()?;
        },
        ContentCommands::Template(cmd) => {
            handle_template_command(cmd)?;
        },
    }
    Ok(())
}

/// Handle topic management commands
fn handle_topic_command(command: TopicCommands) -> Result<()> {
    match command {
        TopicCommands::List => {
            println!("Listing all topics:");
            tools::list_topics()
        },
        TopicCommands::Add {
            key,
            name,
            description,
            path,
        } => {
            println!("Adding new topic: {:?}", name);
            tools::add_topic(key, name, description, path)
        },
        TopicCommands::Edit {
            key,
            name,
            description,
        } => {
            println!("Editing topic: {:?}", key);
            tools::edit_topic(key, name, description)
        },
        TopicCommands::Rename {
            key,
            new_key,
            new_name,
            new_path,
        } => {
            println!("Renaming topic: {:?} to {:?}", key, new_key);
            tools::rename_topic(key, new_key, new_name, new_path)
        },
        TopicCommands::Delete {
            key,
            target,
            force,
        } => {
            println!("Deleting topic: {:?}", key);
            tools::delete_topic(key, target, force)
        },
    }
}

/// Handle image management commands
fn handle_image_command(command: ImageCommands) -> Result<()> {
    match command {
        ImageCommands::Optimize {
            source,
            article,
            topic,
        } => {
            tools::optimize_image(source, article, topic)?;
        }
        ImageCommands::Build {
            article: _,
            topic: _,
            force: _,
        } => {
            println!("Image build not implemented yet");
        }
    }
    Ok(())
}

/// Handle build and generation commands
fn handle_build_command(command: BuildCommands) -> Result<()> {
    match command {
        BuildCommands::Content {
            output_dir,
            slug,
            topic,
            include_drafts,
            skip_html,
            skip_json,
            skip_rss,
            skip_sitemap,
            verbose,
        } => {
            println!("Building content");
            tools::build_content(output_dir, slug, topic, include_drafts, skip_html, skip_json, skip_rss, skip_sitemap, verbose)
        },
        BuildCommands::Toc {
            output,
        } => {
            println!("Generating table of contents");
            tools::generate_toc(output)
        },
        BuildCommands::Llms {
            site_url,
            output_dir,
            include_drafts,
        } => {
            println!("Generating LLMs files");
            tools::generate_llms(site_url, output_dir, include_drafts)
        },
    }
}

/// Handle template management commands
fn handle_template_command(command: &TemplateCommands) -> Result<()> {
    match command {
        TemplateCommands::List => {
            println!("Listing available templates...");
            tools::list_templates()?;
        },
        TemplateCommands::Create { name, content_type } => {
            println!("Creating new template...");
            tools::create_template(name.clone(), content_type.clone())?;
        },
    }
    Ok(())
}

fn content_menu() -> Result<()> {
    let term = Term::stdout();
    
    println!("\n{}", "Content Management".cyan().bold());
    println!("═════════════════════\n");

    let options = vec![
        "Create New Content",
        "Edit Existing Content",
        "Move Content",
        "Delete Content",
        "List All Content",
        "Manage Templates",
        "Back to Main Menu",
    ];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select an option")
        .default(0)
        .items(&options)
        .interact_on_opt(&term)?;

    match selection {
        Some(0) => {
            // Create new content
            let title: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Content title")
                .interact_text()?;

            let topic: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Topic")
                .interact_text()?;

            let tagline: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Tagline (optional)")
                .allow_empty(true)
                .interact_text()?;

            let tags: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Tags (comma-separated, optional)")
                .allow_empty(true)
                .interact_text()?;

            let content_types = vec!["article", "note", "tutorial"];
            let content_type_selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Content type")
                .default(0)
                .items(&content_types)
                .interact()?;

            let content_type = content_types[content_type_selection].to_string();

            let draft = Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("Create as draft?")
                .default(true)
                .interact()?;

            // Ask if user wants to use a template
            let use_template = Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("Use a template?")
                .default(false)
                .interact()?;

            let mut template = None;
            if use_template {
                // Get available templates
                let templates = common_templates::list_templates()?;
                
                if templates.is_empty() {
                    println!("{}", "No templates available for this content type.".yellow());
                } else {
                    // Filter templates by content type
                    let filtered_templates: Vec<_> = templates.into_iter()
                        .filter(|t| t.content_type == content_type)
                        .collect();
                    
                    if filtered_templates.is_empty() {
                        println!("{}", "No templates available for this content type.".yellow());
                    } else {
                        let template_names: Vec<String> = filtered_templates.iter()
                            .map(|t| t.name.clone())
                            .collect();
                        
                        let template_selection = Select::with_theme(&ColorfulTheme::default())
                            .with_prompt("Select a template")
                            .default(0)
                            .items(&template_names)
                            .interact()?;
                        
                        template = Some(template_names[template_selection].clone());
                    }
                }
            }

            // Ask for introduction text
            let introduction: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Introduction text (optional)")
                .allow_empty(true)
                .interact_text()?;

            let introduction_opt = if introduction.is_empty() { None } else { Some(introduction) };

            // Show a progress spinner
            let pb = ProgressBar::new_spinner();
            pb.set_style(
                ProgressStyle::default_spinner()
                    .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ")
                    .template("{spinner:.green} {msg}")
                    .unwrap(),
            );
            pb.set_message("Creating content...");
            pb.enable_steady_tick(Duration::from_millis(100));

            // Call the content creation function
            let tagline_opt = if tagline.is_empty() { None } else { Some(tagline) };
            let tags_opt = if tags.is_empty() { None } else { Some(tags) };
            tools::create_content(Some(title), Some(topic), tagline_opt, tags_opt, Some(content_type), draft, template, introduction_opt)?;

            pb.finish_with_message("Content created successfully!");
            std::thread::sleep(Duration::from_secs(1));
        },
        Some(1) => {
            // Edit content
            let slug: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Slug of the content to edit (leave empty to list available content)")
                .allow_empty(true)
                .interact_text()?;

            let slug_opt = if slug.is_empty() { None } else { Some(slug) };

            // Load topics from config
            let topics = config::get_topics().unwrap_or_else(|_| vec!["default".to_string()]);
            let topic_selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Select a topic (optional)")
                .default(0)
                .items(&topics)
                .interact_on_opt(&term)?;

            let _topic = match topic_selection {
                Some(index) => Some(topics[index].to_string()),
                None => None,
            };

            let edit_options = vec!["Edit frontmatter", "Edit content", "Edit both"];
            let edit_selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("What do you want to edit?")
                .default(2)
                .items(&edit_options)
                .interact()?;

            let frontmatter = edit_selection == 0 || edit_selection == 2;
            let content = edit_selection == 1 || edit_selection == 2;

            // Show a progress spinner
            let pb = ProgressBar::new_spinner();
            pb.set_style(
                ProgressStyle::default_spinner()
                    .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ")
                    .template("{spinner:.green} {msg}")
                    .unwrap(),
            );
            pb.set_message("Editing content...");
            pb.enable_steady_tick(Duration::from_millis(100));

            // Call the content edit function
            tools::edit_content(slug_opt, _topic, frontmatter, content)?;

            pb.finish_with_message("Content edited successfully!");
            std::thread::sleep(Duration::from_secs(1));
        },
        Some(2) => {
            // Move content
            let slug: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Current slug")
                .interact_text()?;

            let new_slug: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("New slug (leave empty to keep the same)")
                .allow_empty(true)
                .interact_text()?;

            // Load topics from config
            let topics = config::get_topics().unwrap_or_else(|_| vec!["default".to_string()]);
            
            let topic_selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Current topic (optional)")
                .default(0)
                .items(&topics)
                .interact_on_opt(&term)?;

            let topic = match topic_selection {
                Some(index) => Some(topics[index].to_string()),
                None => None,
            };

            let new_topic_selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("New topic (optional)")
                .default(0)
                .items(&topics)
                .interact_on_opt(&term)?;

            let new_topic = match new_topic_selection {
                Some(index) => Some(topics[index].to_string()),
                None => None,
            };

            // Show a progress spinner
            let pb = ProgressBar::new_spinner();
            pb.set_style(
                ProgressStyle::default_spinner()
                    .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ")
                    .template("{spinner:.green} {msg}")
                    .unwrap(),
            );
            pb.set_message("Moving content...");
            pb.enable_steady_tick(Duration::from_millis(100));

            // Call the content move function
            let topic_string = match topic_selection {
                Some(index) => topics[index].clone(),
                None => "default".to_string(),
            };
            
            tools::move_content(
                Some(slug),
                if new_slug.is_empty() { None } else { Some(new_slug) },
                Some(topic_string),
                new_topic
            )?;

            pb.finish_with_message("Content moved successfully!");
            std::thread::sleep(Duration::from_secs(1));
        },
        Some(3) => {
            // Delete content
            let slug: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Slug of the content to delete")
                .interact_text()?;

            // Load topics from config
            let topics = config::get_topics().unwrap_or_else(|_| vec!["default".to_string()]);
            let topic_selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Select a topic (optional)")
                .default(0)
                .items(&topics)
                .interact_on_opt(&term)?;

            let topic = match topic_selection {
                Some(index) => Some(topics[index].to_string()),
                None => None,
            };

            let force = Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("Skip confirmation? (This will delete without additional confirmation)")
                .default(false)
                .interact()?;

            // Confirm deletion
            if !force {
                let confirm = Confirm::with_theme(&ColorfulTheme::default())
                    .with_prompt(format!("Are you sure you want to delete '{}'?", slug))
                    .default(false)
                    .interact()?;
                    
                if !confirm {
                    println!("Deletion cancelled.");
                    std::thread::sleep(Duration::from_secs(1));
                    return Ok(());
                }
            }
            
            // Show a progress spinner
            let pb = ProgressBar::new_spinner();
            pb.set_style(
                ProgressStyle::default_spinner()
                    .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ")
                    .template("{spinner:.green} {msg}")
                    .unwrap(),
            );
            pb.set_message("Deleting content...");
            pb.enable_steady_tick(Duration::from_millis(100));
            
            // Call the content delete function
            tools::delete_content(Some(slug), topic, force)?;
            
            pb.finish_with_message("Content deleted successfully!");
            std::thread::sleep(Duration::from_secs(1));
        },
        Some(4) => {
            // List all content
            println!("\n{}", "Listing all content:".yellow());
            
            tools::list_content()?;
            
            println!("\nPress Enter to continue...");
            Term::stdout().read_line()?;
        },
        Some(5) => {
            // Manage templates
            templates_menu()?;
        },
        Some(6) => {}, // Back to main menu
        _ => unreachable!(),
    }

    Ok(())
}

fn templates_menu() -> Result<()> {
    let term = Term::stdout();
    
    println!("\n{}", "Template Management".cyan().bold());
    println!("═══════════════════\n");

    let options = vec![
        "List Templates",
        "Create New Template",
        "Back to Content Menu",
    ];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select an option")
        .default(0)
        .items(&options)
        .interact_on_opt(&term)?;

    match selection {
        Some(0) => {
            // List templates
            println!("\n{}", "Available Templates:".yellow());
            tools::list_templates()?;
            
            println!("\nPress Enter to continue...");
            Term::stdout().read_line()?;
        },
        Some(1) => {
            // Create new template
            let name: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Template name")
                .interact_text()?;

            let content_types = vec!["article", "note", "tutorial"];
            let content_type_selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Content type")
                .default(0)
                .items(&content_types)
                .interact()?;

            let content_type = content_types[content_type_selection].to_string();

            // Show a progress spinner
            let pb = ProgressBar::new_spinner();
            pb.set_style(
                ProgressStyle::default_spinner()
                    .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ")
                    .template("{spinner:.green} {msg}")
                    .unwrap(),
            );
            pb.set_message("Creating template...");
            pb.enable_steady_tick(Duration::from_millis(100));

            // Call the template creation function
            tools::create_template(Some(name), Some(content_type))?;

            pb.finish_with_message("Template created successfully!");
            std::thread::sleep(Duration::from_secs(1));
        },
        Some(2) => {
            // Back to content menu
            return content_menu();
        },
        _ => unreachable!(),
    }

    Ok(())
}

fn topics_menu() -> Result<()> {
    let term = Term::stdout();
    
    println!("\n{}", "Topic Management".cyan().bold());
    println!("═══════════════════\n");

    let options = vec![
        "List All Topics",
        "Add New Topic",
        "Edit Topic",
        "Rename Topic",
        "Delete Topic",
        "Back to Main Menu",
    ];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select an option")
        .default(0)
        .items(&options)
        .interact_on_opt(&term)?;

    match selection {
        Some(0) => {
            // List all topics
            println!("\n{}", "Listing all topics:".yellow());
            
            tools::list_topics()?;
            
            println!("\nPress Enter to continue...");
            Term::stdout().read_line()?;
        },
        Some(1) => {
            // Add new topic
            let key: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Topic key (used in URLs)")
                .interact_text()?;

            let name: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Topic name")
                .interact_text()?;

            let description: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Topic description")
                .interact_text()?;

            let path: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Topic path (leave empty to use key)")
                .allow_empty(true)
                .interact_text()?;

            // Show a progress spinner
            let pb = ProgressBar::new_spinner();
            pb.set_style(
                ProgressStyle::default_spinner()
                    .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ")
                    .template("{spinner:.green} {msg}")
                    .unwrap(),
            );
            pb.set_message("Adding new topic...");
            pb.enable_steady_tick(Duration::from_millis(100));

            // Call the topic add function
            let path_opt = if path.is_empty() { None } else { Some(path) };
            tools::add_topic(Some(key), Some(name), Some(description), path_opt)?;

            pb.finish_with_message("Topic added successfully!");
            std::thread::sleep(Duration::from_secs(1));
        },
        Some(2) => {
            // Edit topic
            // Load topics from config for selection
            let topics = config::get_topics().unwrap_or_else(|_| vec!["default".to_string()]);
            
            let topic_selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Select a topic to edit")
                .default(0)
                .items(&topics)
                .interact()?;
            
            let key = topics[topic_selection].to_string();
            
            let name: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("New name (leave empty to keep current)")
                .allow_empty(true)
                .interact_text()?;
                
            let description: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("New description (leave empty to keep current)")
                .allow_empty(true)
                .interact_text()?;
                
            // Show a progress spinner
            let pb = ProgressBar::new_spinner();
            pb.set_style(
                ProgressStyle::default_spinner()
                    .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ")
                    .template("{spinner:.green} {msg}")
                    .unwrap(),
            );
            pb.set_message("Editing topic...");
            pb.enable_steady_tick(Duration::from_millis(100));
            
            // Call the topic edit function
            let name_opt = if name.is_empty() { None } else { Some(name) };
            let desc_opt = if description.is_empty() { None } else { Some(description) };
            tools::edit_topic(Some(key), name_opt, desc_opt)?;
            
            pb.finish_with_message("Topic edited successfully!");
            std::thread::sleep(Duration::from_secs(1));
        },
        Some(3) => {
            // Rename topic
            // Load topics from config for selection
            let topics = config::get_topics().unwrap_or_else(|_| vec!["default".to_string()]);
            
            let topic_selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Select a topic to rename")
                .default(0)
                .items(&topics)
                .interact()?;
            
            let key = topics[topic_selection].to_string();
            
            let new_key: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("New key (leave empty to keep current)")
                .allow_empty(true)
                .interact_text()?;
                
            let new_name: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("New name (leave empty to keep current)")
                .allow_empty(true)
                .interact_text()?;
                
            let new_path: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("New path (leave empty to keep current)")
                .allow_empty(true)
                .interact_text()?;
                
            // Show a progress spinner
            let pb = ProgressBar::new_spinner();
            pb.set_style(
                ProgressStyle::default_spinner()
                    .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ")
                    .template("{spinner:.green} {msg}")
                    .unwrap(),
            );
            pb.set_message("Renaming topic...");
            pb.enable_steady_tick(Duration::from_millis(100));
            
            // Call the topic rename function
            let new_key_opt = if new_key.is_empty() { None } else { Some(new_key) };
            let new_name_opt = if new_name.is_empty() { None } else { Some(new_name) };
            let new_path_opt = if new_path.is_empty() { None } else { Some(new_path) };
            tools::rename_topic(Some(key), new_key_opt, new_name_opt, new_path_opt)?;
            
            pb.finish_with_message("Topic renamed successfully!");
            std::thread::sleep(Duration::from_secs(1));
        },
        Some(4) => {
            // Delete topic
            // Load topics from config for selection
            let topics = config::get_topics().unwrap_or_else(|_| vec!["default".to_string()]);
            
            let topic_selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Select a topic to delete")
                .default(0)
                .items(&topics)
                .interact()?;
            
            let key = topics[topic_selection].to_string();
            
            // If there are other topics, offer to migrate content
            let mut target_opt = None;
            if topics.len() > 1 {
                let other_topics: Vec<String> = topics.iter()
                    .filter(|t| **t != key)
                    .cloned()
                    .collect();
                
                let migrate = Confirm::with_theme(&ColorfulTheme::default())
                    .with_prompt("Migrate content to another topic?")
                    .default(true)
                    .interact()?;
                    
                if migrate {
                    let target_selection = Select::with_theme(&ColorfulTheme::default())
                        .with_prompt("Select target topic for migration")
                        .default(0)
                        .items(&other_topics)
                        .interact()?;
                        
                    target_opt = Some(other_topics[target_selection].to_string());
                }
            }
            
            let force = Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("Skip confirmation? (This will delete without additional confirmation)")
                .default(false)
                .interact()?;
                
            // Confirm deletion
            if !force {
                let confirm = Confirm::with_theme(&ColorfulTheme::default())
                    .with_prompt(format!("Are you sure you want to delete the '{}' topic?", key))
                    .default(false)
                    .interact()?;
                    
                if !confirm {
                    println!("Deletion cancelled.");
                    std::thread::sleep(Duration::from_secs(1));
                    return Ok(());
                }
            }
            
            // Show a progress spinner
            let pb = ProgressBar::new_spinner();
            pb.set_style(
                ProgressStyle::default_spinner()
                    .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ")
                    .template("{spinner:.green} {msg}")
                    .unwrap(),
            );
            pb.set_message("Deleting topic...");
            pb.enable_steady_tick(Duration::from_millis(100));
            
            // Call the topic delete function
            tools::delete_topic(Some(key), target_opt, force)?;
            
            pb.finish_with_message("Topic deleted successfully!");
            std::thread::sleep(Duration::from_secs(1));
        },
        Some(5) => {}, // Back to main menu
        _ => unreachable!(),
    }

    Ok(())
}

fn images_menu() -> Result<()> {
    let term = Term::stdout();
    
    println!("\n{}", "Image Management".cyan().bold());
    println!("════════════════════\n");

    let options = vec![
        "Optimize Image",
        "Build Image Variants (not implemented)",
        "Back to Main Menu",
    ];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select an option")
        .default(0)
        .items(&options)
        .interact_on_opt(&term)?;

    match selection {
        Some(0) => {
            // Optimize image
            let source: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Source image path")
                .interact_text()?;

            let article: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Article slug")
                .interact_text()?;

            // Load topics from config for selection
            let topics = config::get_topics().unwrap_or_else(|_| vec!["default".to_string()]);
            
            let topic_selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Select a topic (optional)")
                .default(0)
                .items(&topics)
                .interact_on_opt(&term)?;
            
            let topic = match topic_selection {
                Some(index) => Some(topics[index].to_string()),
                None => None,
            };

            // Show a progress spinner
            let pb = ProgressBar::new_spinner();
            pb.set_style(
                ProgressStyle::default_spinner()
                    .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ")
                    .template("{spinner:.green} {msg}")
                    .unwrap(),
            );
            pb.set_message("Optimizing image...");
            pb.enable_steady_tick(Duration::from_millis(100));

            // Call the image optimize function
            tools::optimize_image(source, article, topic)?;

            pb.finish_with_message("Image optimized successfully!");
            std::thread::sleep(Duration::from_secs(1));
        },
        Some(1) => {
            // Build image variants
            println!("\nThis feature is not yet implemented.");
            std::thread::sleep(Duration::from_secs(2));
        },
        Some(2) => {}, // Back to main menu
        _ => unreachable!(),
    }

    Ok(())
}

fn build_menu() -> Result<()> {
    let term = Term::stdout();
    
    println!("\n{}", "Build & Generate".cyan().bold());
    println!("════════════════════\n");

    let options = vec![
        "Build Content",
        "Generate Table of Contents",
        "Generate LLMs Files",
        "Back to Main Menu",
    ];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select an option")
        .default(0)
        .items(&options)
        .interact_on_opt(&term)?;

    match selection {
        Some(0) => {
            // Build content
            let output_dir: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Output directory (leave empty for default)")
                .allow_empty(true)
                .interact_text()?;

            let slug: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Specific content slug (leave empty for all)")
                .allow_empty(true)
                .interact_text()?;

            // Load topics from config for selection
            let topics = config::get_topics().unwrap_or_else(|_| vec!["default".to_string()]);
            let all_topics = {
                let mut t = vec!["All topics".to_string()];
                t.extend(topics.clone());
                t
            };
            
            let topic_selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Select a topic (or 'All topics')")
                .default(0)
                .items(&all_topics)
                .interact()?;
            
            let topic = if topic_selection == 0 {
                None
            } else {
                Some(topics[topic_selection - 1].to_string())
            };

            let include_drafts = Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("Include drafts?")
                .default(false)
                .interact()?;

            let skip_html = Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("Skip HTML generation?")
                .default(false)
                .interact()?;

            let skip_json = Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("Skip JSON generation?")
                .default(false)
                .interact()?;

            let skip_rss = Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("Skip RSS generation?")
                .default(false)
                .interact()?;

            let skip_sitemap = Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("Skip sitemap generation?")
                .default(false)
                .interact()?;

            let verbose = Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("Show verbose output?")
                .default(true)
                .interact()?;

            // Show a progress spinner
            let pb = ProgressBar::new_spinner();
            pb.set_style(
                ProgressStyle::default_spinner()
                    .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ")
                    .template("{spinner:.green} {msg}")
                    .unwrap(),
            );
            pb.set_message("Building content...");
            pb.enable_steady_tick(Duration::from_millis(100));

            // Call the content build function
            let output_dir_opt = if output_dir.is_empty() { None } else { Some(output_dir) };
            let slug_opt = if slug.is_empty() { None } else { Some(slug) };
            tools::build_content(
                output_dir_opt,
                slug_opt,
                topic,
                include_drafts,
                skip_html,
                skip_json,
                skip_rss,
                skip_sitemap,
                verbose,
            )?;

            pb.finish_with_message("Content built successfully!");
            std::thread::sleep(Duration::from_secs(1));
        },
        Some(1) => {
            // Generate table of contents
            let output: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Output file path (leave empty for default)")
                .allow_empty(true)
                .interact_text()?;

            // Show a progress spinner
            let pb = ProgressBar::new_spinner();
            pb.set_style(
                ProgressStyle::default_spinner()
                    .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ")
                    .template("{spinner:.green} {msg}")
                    .unwrap(),
            );
            pb.set_message("Generating table of contents...");
            pb.enable_steady_tick(Duration::from_millis(100));

            // Call the TOC generation function
            let output_opt = if output.is_empty() { None } else { Some(output) };
            tools::generate_toc(output_opt)?;

            pb.finish_with_message("Table of contents generated successfully!");
            std::thread::sleep(Duration::from_secs(1));
        },
        Some(2) => {
            // Generate LLMs files
            let site_url: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Site URL (required)")
                .interact_text()?;

            let output_dir: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Output directory (leave empty for default)")
                .allow_empty(true)
                .interact_text()?;

            let include_drafts = Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("Include drafts?")
                .default(false)
                .interact()?;

            // Show a progress spinner
            let pb = ProgressBar::new_spinner();
            pb.set_style(
                ProgressStyle::default_spinner()
                    .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ")
                    .template("{spinner:.green} {msg}")
                    .unwrap(),
            );
            pb.set_message("Generating LLMs files...");
            pb.enable_steady_tick(Duration::from_millis(100));

            // Call the LLMs generation function
            let output_dir_opt = if output_dir.is_empty() { None } else { Some(output_dir) };
            tools::generate_llms(Some(site_url), output_dir_opt, include_drafts)?;

            pb.finish_with_message("LLMs files generated successfully!");
            std::thread::sleep(Duration::from_secs(1));
        },
        Some(3) => {}, // Back to main menu
        _ => unreachable!(),
    }

    Ok(())
}

fn stats_menu() -> Result<()> {
    let _term = Term::stdout();
    
    println!("\n{}", "Content Statistics".cyan().bold());
    println!("═══════════════════\n");

    // Load topics from config for selection
    let topics = config::get_topics().unwrap_or_else(|_| vec!["default".to_string()]);
    let all_topics = {
        let mut t = vec!["All topics".to_string()];
        t.extend(topics.clone());
        t
    };
    
    let topic_selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a topic (or 'All topics')")
        .default(0)
        .items(&all_topics)
        .interact()?;
    
    let topic = if topic_selection == 0 {
        None
    } else {
        Some(topics[topic_selection - 1].to_string())
    };

    let slug: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Specific content slug (leave empty for all)")
        .allow_empty(true)
        .interact_text()?;

    let include_drafts = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Include drafts?")
        .default(false)
        .interact()?;

    let sort_options = vec!["Date", "Word count", "Reading time"];
    let sort_selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Sort by")
        .default(0)
        .items(&sort_options)
        .interact()?;

    let sort_by = match sort_selection {
        0 => "date",
        1 => "words",
        2 => "time",
        _ => "date",
    };

    let detailed = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Show detailed statistics?")
        .default(true)
        .interact()?;

    // Show a progress spinner
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ")
            .template("{spinner:.green} {msg}")
            .unwrap(),
    );
    pb.set_message("Generating statistics...");
    pb.enable_steady_tick(Duration::from_millis(100));

    // Call the stats function
    let slug_opt = if slug.is_empty() { None } else { Some(slug) };
    tools::generate_content_stats(slug_opt, topic, include_drafts, sort_by.to_string(), detailed)?;

    pb.finish();
    
    println!("\nPress Enter to continue...");
    Term::stdout().read_line()?;

    Ok(())
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Some(command) => match command {
            Commands::Content(cmd) => handle_content_command(&cmd)?,
            Commands::Topic(cmd) => handle_topic_command(cmd)?,
            Commands::Image(cmd) => handle_image_command(cmd)?,
            Commands::Build(cmd) => handle_build_command(cmd)?,
            Commands::Stats { 
                slug, 
                topic, 
                include_drafts, 
                sort_by, 
                detailed 
            } => {
                tools::generate_content_stats(
                    slug, 
                    topic, 
                    include_drafts, 
                    sort_by, 
                    detailed
                )?;
            }
        },
        None => {
            run_interactive_cli()?;
        }
    }

    Ok(())
}
