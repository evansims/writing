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
    Frame, Terminal,
};

mod tools;
mod config;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Launch the interactive TUI
    Interactive,
    /// Create new content
    New {
        #[arg(long)]
        title: Option<String>,
        #[arg(long)]
        topic: Option<String>,
        #[arg(long)]
        tagline: Option<String>,
        #[arg(long)]
        tags: Option<String>,
        #[arg(long)]
        content_type: Option<String>,
        #[arg(long)]
        draft: bool,
    },
    /// Edit existing content
    Edit {
        #[arg(long)]
        slug: Option<String>,
        #[arg(long)]
        topic: Option<String>,
        #[arg(long)]
        frontmatter: bool,
        #[arg(long)]
        content: bool,
    },
    /// Move content (change slug and/or topic)
    Move {
        #[arg(long)]
        slug: Option<String>,
        #[arg(long)]
        new_slug: Option<String>,
        #[arg(long)]
        topic: Option<String>,
        #[arg(long)]
        new_topic: Option<String>,
    },
    /// Delete content
    Delete {
        #[arg(long)]
        slug: Option<String>,
        #[arg(long)]
        topic: Option<String>,
        #[arg(long)]
        force: bool,
    },
    /// List all content
    List,
    /// List all topics
    Topics,
    /// Generate content statistics
    Stats {
        #[arg(long)]
        slug: Option<String>,
        #[arg(long)]
        topic: Option<String>,
        #[arg(long)]
        include_drafts: bool,
        #[arg(long)]
        sort_by: Option<String>,
        #[arg(long)]
        detailed: bool,
    },
    /// Add a new topic
    TopicAdd {
        #[arg(long)]
        key: Option<String>,
        #[arg(long)]
        name: Option<String>,
        #[arg(long)]
        description: Option<String>,
        #[arg(long)]
        path: Option<String>,
    },
    /// Edit an existing topic
    TopicEdit {
        #[arg(long)]
        key: Option<String>,
        #[arg(long)]
        name: Option<String>,
        #[arg(long)]
        description: Option<String>,
    },
    /// Rename a topic
    TopicRename {
        #[arg(long)]
        key: Option<String>,
        #[arg(long)]
        new_key: Option<String>,
        #[arg(long)]
        new_name: Option<String>,
        #[arg(long)]
        new_path: Option<String>,
    },
    /// Delete a topic
    TopicDelete {
        #[arg(long)]
        key: Option<String>,
        #[arg(long)]
        target: Option<String>,
        #[arg(long)]
        force: bool,
    },
    /// Optimize an image
    ImageOptimize {
        #[arg(long)]
        source: String,
        #[arg(long)]
        article: String,
        #[arg(long)]
        topic: Option<String>,
    },
    /// Generate table of contents
    Toc {
        #[arg(long)]
        output: Option<String>,
    },
    /// Generate LLMs files
    Llms {
        #[arg(long)]
        site_url: Option<String>,
        #[arg(long)]
        output_dir: Option<String>,
        #[arg(long)]
        include_drafts: bool,
    },
    /// Build content
    Build {
        #[arg(long)]
        output_dir: Option<String>,
        #[arg(long)]
        slug: Option<String>,
        #[arg(long)]
        topic: Option<String>,
        #[arg(long)]
        include_drafts: bool,
        #[arg(long)]
        template_dir: Option<String>,
        #[arg(long)]
        site_url: Option<String>,
    },
}

enum MenuItem {
    Content,
    Topics,
    Images,
    Build,
    Stats,
    Quit,
}

enum ContentMenuItem {
    New,
    Edit,
    Move,
    Delete,
    List,
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
                    MenuItem::Quit => MenuItem::Stats,
                };
                false
            }
            KeyCode::Down => {
                self.selected_menu_item = match self.selected_menu_item {
                    MenuItem::Content => MenuItem::Topics,
                    MenuItem::Topics => MenuItem::Images,
                    MenuItem::Images => MenuItem::Build,
                    MenuItem::Build => MenuItem::Stats,
                    MenuItem::Stats => MenuItem::Quit,
                    MenuItem::Quit => MenuItem::Content,
                };
                false
            }
            KeyCode::Enter => {
                match self.selected_menu_item {
                    MenuItem::Content => self.menu_state = MenuState::Content,
                    MenuItem::Topics => self.menu_state = MenuState::Topics,
                    MenuItem::Images => self.menu_state = MenuState::Images,
                    MenuItem::Build => self.menu_state = MenuState::Build,
                    MenuItem::Stats => {
                        // Handle stats directly
                        return true;
                    }
                    MenuItem::Quit => return true,
                }
                false
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
                    ContentMenuItem::List => ContentMenuItem::Back,
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

fn ui(f: &mut Frame, app: &App) {
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

fn render_main_menu(f: &mut Frame, app: &App, area: tui::layout::Rect) {
    let items = vec![
        ListItem::new("Content Management"),
        ListItem::new("Topics Management"),
        ListItem::new("Images Management"),
        ListItem::new("Build Operations"),
        ListItem::new("Content Statistics"),
        ListItem::new("Quit"),
    ];

    let selected_index = match app.selected_menu_item {
        MenuItem::Content => 0,
        MenuItem::Topics => 1,
        MenuItem::Images => 2,
        MenuItem::Build => 3,
        MenuItem::Stats => 4,
        MenuItem::Quit => 5,
    };

    let list = List::new(items)
        .block(Block::default().title("Main Menu").borders(Borders::ALL))
        .highlight_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        .highlight_symbol(">> ");

    let mut state = tui::widgets::ListState::default();
    state.select(Some(selected_index));
    f.render_stateful_widget(list, area, &mut state);
}

fn render_content_menu(f: &mut Frame, app: &App, area: tui::layout::Rect) {
    let items = vec![
        ListItem::new("Create New Content"),
        ListItem::new("Edit Existing Content"),
        ListItem::new("Move Content"),
        ListItem::new("Delete Content"),
        ListItem::new("List All Content"),
        ListItem::new("Back to Main Menu"),
    ];

    let selected_index = match app.selected_content_menu_item {
        ContentMenuItem::New => 0,
        ContentMenuItem::Edit => 1,
        ContentMenuItem::Move => 2,
        ContentMenuItem::Delete => 3,
        ContentMenuItem::List => 4,
        ContentMenuItem::Back => 5,
    };

    let list = List::new(items)
        .block(Block::default().title("Content Management").borders(Borders::ALL))
        .highlight_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        .highlight_symbol(">> ");

    let mut state = tui::widgets::ListState::default();
    state.select(Some(selected_index));
    f.render_stateful_widget(list, area, &mut state);
}

fn render_topics_menu(f: &mut Frame, app: &App, area: tui::layout::Rect) {
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

fn render_images_menu(f: &mut Frame, app: &App, area: tui::layout::Rect) {
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

fn render_build_menu(f: &mut Frame, app: &App, area: tui::layout::Rect) {
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
    loop {
        let term = Term::stdout();
        term.clear_screen()?;

        println!("{}", "Writing Management System".cyan().bold());
        println!("{}", "=========================".cyan());
        println!();

        let options = vec![
            "Content Management",
            "Topics Management",
            "Images Management",
            "Build Operations",
            "Content Statistics",
            "Quit",
        ];

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select an option")
            .default(0)
            .items(&options)
            .interact_on_opt(&term)?;

        match selection {
            Some(0) => handle_content_menu()?,
            Some(1) => handle_topics_menu()?,
            Some(2) => handle_images_menu()?,
            Some(3) => handle_build_menu()?,
            Some(4) => handle_stats_menu()?,
            Some(5) | None => break,
            _ => unreachable!(),
        }
    }

    Ok(())
}

fn handle_content_menu() -> Result<()> {
    let term = Term::stdout();
    term.clear_screen()?;

    println!("{}", "Content Management".cyan().bold());
    println!("{}", "=================".cyan());
    println!();

    let options = vec![
        "Create New Content",
        "Edit Existing Content",
        "Move Content",
        "Delete Content",
        "List All Content",
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
                .with_prompt("Title")
                .interact_text()?;

            // Load topics from config
            let topics = config::get_topics().unwrap_or_else(|_| vec!["default".to_string()]);
            let topic_selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Select a topic")
                .default(0)
                .items(&topics)
                .interact_on_opt(&term)?;

            let topic = match topic_selection {
                Some(index) => Some(topics[index].to_string()),
                None => None,
            };

            let tagline: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Tagline (optional)")
                .allow_empty(true)
                .interact_text()?;

            let tags: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Tags (comma separated, optional)")
                .allow_empty(true)
                .interact_text()?;

            let content_types = vec!["article", "note"];
            let content_type_selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Content type")
                .default(0)
                .items(&content_types)
                .interact_on_opt(&term)?;

            let content_type = match content_type_selection {
                Some(index) => Some(content_types[index].to_string()),
                None => None,
            };

            let draft = Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("Create as draft?")
                .default(false)
                .interact()?;

            // Show a progress bar
            let pb = ProgressBar::new_spinner();
            pb.set_style(
                ProgressStyle::default_spinner()
                    .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ")
                    .template("{spinner:.green} {msg}")
                    .unwrap(),
            );
            pb.set_message("Creating new content...");
            pb.enable_steady_tick(Duration::from_millis(100));

            // Call the content-new tool
            let tagline_opt = if tagline.is_empty() { None } else { Some(tagline) };
            let tags_opt = if tags.is_empty() { None } else { Some(tags) };
            tools::create_content(Some(title), topic, tagline_opt, tags_opt, content_type, draft)?;

            pb.finish_with_message("Content created successfully!");
            std::thread::sleep(Duration::from_secs(1));
        }
        Some(1) => {
            // Edit content
            let slug: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Slug of the content to edit")
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

            let edit_options = vec!["Edit frontmatter", "Edit content", "Edit both"];
            let edit_selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("What do you want to edit?")
                .default(2)
                .items(&edit_options)
                .interact()?;

            let frontmatter = edit_selection == 0 || edit_selection == 2;
            let content = edit_selection == 1 || edit_selection == 2;

            // Show a progress bar
            let pb = ProgressBar::new_spinner();
            pb.set_style(
                ProgressStyle::default_spinner()
                    .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ")
                    .template("{spinner:.green} {msg}")
                    .unwrap(),
            );
            pb.set_message("Editing content...");
            pb.enable_steady_tick(Duration::from_millis(100));

            // Call the content-edit tool
            tools::edit_content(Some(slug), topic, frontmatter, content)?;

            pb.finish_with_message("Content edited successfully!");
            std::thread::sleep(Duration::from_secs(1));
        }
        Some(2) => {
            // Move content
            let slug: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Current slug")
                .interact_text()?;

            let new_slug: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("New slug (leave empty to keep the same)")
                .allow_empty(true)
                .interact_text()?;

            let topics = vec!["strategy", "mindset", "productivity", "focus"]; // This should be loaded from config
            
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

            // Show a progress bar
            let pb = ProgressBar::new_spinner();
            pb.set_style(
                ProgressStyle::default_spinner()
                    .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ")
                    .template("{spinner:.green} {msg}")
                    .unwrap(),
            );
            pb.set_message("Moving content...");
            pb.enable_steady_tick(Duration::from_millis(100));

            // Call the content-move tool
            let new_slug_opt = if new_slug.is_empty() { None } else { Some(new_slug) };
            tools::move_content(Some(slug), new_slug_opt, topic, new_topic)?;

            pb.finish_with_message("Content moved successfully!");
            std::thread::sleep(Duration::from_secs(1));
        }
        Some(3) => {
            // Delete content
            let slug: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Slug of the content to delete")
                .interact_text()?;

            let topics = vec!["strategy", "mindset", "productivity", "focus"]; // This should be loaded from config
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
                .with_prompt("Force deletion? (This will delete without confirmation)")
                .default(false)
                .interact()?;

            // Confirm deletion
            if !force {
                let confirm = Confirm::with_theme(&ColorfulTheme::default())
                    .with_prompt("Are you sure you want to delete this content?")
                    .default(false)
                    .interact()?;

                if !confirm {
                    println!("Deletion cancelled.");
                    std::thread::sleep(Duration::from_secs(1));
                    return handle_content_menu();
                }
            }

            // Show a progress bar
            let pb = ProgressBar::new_spinner();
            pb.set_style(
                ProgressStyle::default_spinner()
                    .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ")
                    .template("{spinner:.green} {msg}")
                    .unwrap(),
            );
            pb.set_message("Deleting content...");
            pb.enable_steady_tick(Duration::from_millis(100));

            // Call the content-delete tool
            tools::delete_content(Some(slug), topic, force)?;

            pb.finish_with_message("Content deleted successfully!");
            std::thread::sleep(Duration::from_secs(1));
        }
        Some(4) => {
            // List all content
            println!("\n{}", "Listing all content:".yellow());
            
            match tools::list_content() {
                Ok(_) => {},
                Err(e) => {
                    eprintln!("Error listing content: {}", e);
                }
            }
            
            // Return to content menu instead of exiting
            return handle_content_menu();
        }
        Some(5) => return Ok(()), // Back to main menu
        _ => {
            // Handle other content management options
            println!("Not implemented yet");
            std::thread::sleep(Duration::from_secs(2));
            return handle_content_menu();
        }
    }

    Ok(())
}

fn handle_topics_menu() -> Result<()> {
    let term = Term::stdout();
    term.clear_screen()?;

    println!("{}", "Topics Management".cyan().bold());
    println!("{}", "=================".cyan());
    println!();

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
            
            match tools::list_topics() {
                Ok(_) => {},
                Err(e) => {
                    eprintln!("Error listing topics: {}", e);
                }
            }
            
            // Return to topics menu instead of exiting
            return handle_topics_menu();
        }
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

            // Show a progress bar
            let pb = ProgressBar::new_spinner();
            pb.set_style(
                ProgressStyle::default_spinner()
                    .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ")
                    .template("{spinner:.green} {msg}")
                    .unwrap(),
            );
            pb.set_message("Adding new topic...");
            pb.enable_steady_tick(Duration::from_millis(100));

            // Call the topic-add tool
            let path_opt = if path.is_empty() { None } else { Some(path) };
            tools::add_topic(Some(key), Some(name), Some(description), path_opt)?;

            pb.finish_with_message("Topic added successfully!");
            std::thread::sleep(Duration::from_secs(1));
            
            // Return to topics menu
            return handle_topics_menu();
        }
        Some(5) => return Ok(()), // Back to main menu
        _ => {
            // Other topic management options
            println!("Not implemented yet");
            std::thread::sleep(Duration::from_secs(2));
            return handle_topics_menu();
        }
    }

    Ok(())
}

fn handle_images_menu() -> Result<()> {
    let term = Term::stdout();
    term.clear_screen()?;

    println!("{}", "Images Management".cyan().bold());
    println!("{}", "=================".cyan());
    println!();

    let options = vec![
        "Optimize Source Image",
        "Build Optimized Images",
        "Back to Main Menu",
    ];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select an option")
        .default(0)
        .items(&options)
        .interact_on_opt(&term)?;

    match selection {
        Some(0) => {
            // Optimize source image
            let source: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Source image path")
                .interact_text()?;

            let article: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Article slug")
                .interact_text()?;

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

            // Show a progress bar
            let pb = ProgressBar::new_spinner();
            pb.set_style(
                ProgressStyle::default_spinner()
                    .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ")
                    .template("{spinner:.green} {msg}")
                    .unwrap(),
            );
            pb.set_message("Optimizing source image...");
            pb.enable_steady_tick(Duration::from_millis(100));

            // Call the optimize tool
            tools::optimize_image(source, article, topic)?;

            pb.finish_with_message("Source image optimized successfully!");
            std::thread::sleep(Duration::from_secs(1));
            
            // Return to images menu
            return handle_images_menu();
        }
        Some(1) => {
            // Build optimized images
            // ... implement this ...
            println!("Not implemented yet");
            std::thread::sleep(Duration::from_secs(2));
            return handle_images_menu();
        }
        Some(2) => return Ok(()), // Back to main menu
        _ => {
            // Other image management options
            println!("Not implemented yet");
            std::thread::sleep(Duration::from_secs(2));
            return handle_images_menu();
        }
    }

    Ok(())
}

fn handle_build_menu() -> Result<()> {
    let term = Term::stdout();
    term.clear_screen()?;

    println!("{}", "Build Operations".cyan().bold());
    println!("{}", "===============".cyan());
    println!();

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
            let site_url: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Site URL (optional)")
                .allow_empty(true)
                .interact_text()?;

            let include_drafts = Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("Include drafts?")
                .default(false)
                .interact()?;

            // Show a progress bar
            let pb = ProgressBar::new_spinner();
            pb.set_style(
                ProgressStyle::default_spinner()
                    .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ")
                    .template("{spinner:.green} {msg}")
                    .unwrap(),
            );
            pb.set_message("Building content...");
            pb.enable_steady_tick(Duration::from_millis(100));

            // Call the content-build tool
            let site_url_opt = if site_url.is_empty() { None } else { Some(site_url) };
            tools::build_content(None, None, None, include_drafts, None, site_url_opt)?;

            pb.finish_with_message("Content built successfully!");
            std::thread::sleep(Duration::from_secs(1));
            
            // Return to build menu
            return handle_build_menu();
        }
        Some(1) => {
            // Generate table of contents
            // ... implement this ...
            println!("Not implemented yet");
            std::thread::sleep(Duration::from_secs(2));
            return handle_build_menu();
        }
        Some(2) => {
            // Generate LLMs files
            // ... implement this ...
            println!("Not implemented yet");
            std::thread::sleep(Duration::from_secs(2));
            return handle_build_menu();
        }
        Some(3) => return Ok(()), // Back to main menu
        _ => {
            // Other build operations
            println!("Not implemented yet");
            std::thread::sleep(Duration::from_secs(2));
            return handle_build_menu();
        }
    }

    Ok(())
}

fn handle_stats_menu() -> Result<()> {
    let term = Term::stdout();
    term.clear_screen()?;

    println!("{}", "Content Statistics".cyan().bold());
    println!("{}", "=================".cyan());
    println!();

    let include_drafts = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Include drafts?")
        .default(false)
        .interact()?;

    let sort_options = vec!["date", "words", "reading_time"];
    let sort_selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Sort by")
        .default(0)
        .items(&sort_options)
        .interact()?;

    let sort_by = sort_options[sort_selection].to_string();

    let detailed = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Show detailed statistics?")
        .default(false)
        .interact()?;

    // Show a progress bar
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ")
            .template("{spinner:.green} {msg}")
            .unwrap(),
    );
    pb.set_message("Generating statistics...");
    pb.enable_steady_tick(Duration::from_millis(100));

    // Call the content stats function to generate statistics
    match tools::generate_content_stats(None, None, include_drafts, sort_by, detailed) {
        Ok(_) => {},
        Err(e) => {
            eprintln!("Error generating statistics: {}", e);
        }
    }
    
    // Return to main menu
    Ok(())
}

fn main() -> Result<()> {
    // Load config on startup to catch any errors early
    match config::load_config() {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Error loading config: {}", e);
            eprintln!("Make sure you're running this command from the root of your writing project.");
            std::process::exit(1);
        }
    }

    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Interactive) => run_interactive_tui()?,
        Some(Commands::New {
            title,
            topic,
            tagline,
            tags,
            content_type,
            draft,
        }) => {
            println!("Creating new content with title: {:?}", title);
            tools::create_content(title, topic, tagline, tags, content_type, draft)?;
        },
        Some(Commands::Edit {
            slug,
            topic,
            frontmatter,
            content,
        }) => {
            println!("Editing content with slug: {:?}", slug);
            tools::edit_content(slug, topic, frontmatter, content)?;
        },
        Some(Commands::Move {
            slug,
            new_slug,
            topic,
            new_topic,
        }) => {
            println!("Moving content with slug: {:?}", slug);
            tools::move_content(slug, new_slug, topic, new_topic)?;
        },
        Some(Commands::Delete {
            slug,
            topic,
            force,
        }) => {
            println!("Deleting content with slug: {:?}", slug);
            tools::delete_content(slug, topic, force)?;
        },
        Some(Commands::List) => {
            println!("Listing all content:");
            tools::list_content()?;
        },
        Some(Commands::Topics) => {
            println!("Listing all topics:");
            tools::list_topics()?;
        },
        Some(Commands::Stats {
            slug,
            topic,
            include_drafts,
            sort_by,
            detailed,
        }) => {
            // Convert the sort_by Option to String with a default
            let sort_by_value = sort_by.unwrap_or_else(|| String::from("date"));
            tools::generate_content_stats(slug, topic, include_drafts, sort_by_value, detailed)?;
        },
        Some(Commands::TopicAdd {
            key,
            name,
            description,
            path,
        }) => {
            println!("Adding new topic: {:?}", name);
            tools::add_topic(key, name, description, path)?;
        },
        Some(Commands::TopicEdit {
            key,
            name,
            description,
        }) => {
            println!("Editing topic: {:?}", key);
            tools::edit_topic(key, name, description)?;
        },
        Some(Commands::TopicRename {
            key,
            new_key,
            new_name,
            new_path,
        }) => {
            println!("Renaming topic: {:?} to {:?}", key, new_key);
            tools::rename_topic(key, new_key, new_name, new_path)?;
        },
        Some(Commands::TopicDelete {
            key,
            target,
            force,
        }) => {
            println!("Deleting topic: {:?}", key);
            tools::delete_topic(key, target, force)?;
        },
        Some(Commands::ImageOptimize {
            source,
            article,
            topic,
        }) => {
            println!("Optimizing image: {:?} for article: {:?}", source, article);
            tools::optimize_image(source, article, topic)?;
        },
        Some(Commands::Toc {
            output,
        }) => {
            println!("Generating table of contents");
            tools::generate_toc(output)?;
        },
        Some(Commands::Llms {
            site_url,
            output_dir,
            include_drafts,
        }) => {
            println!("Generating LLMs files");
            tools::generate_llms(site_url, output_dir, include_drafts)?;
        },
        Some(Commands::Build {
            output_dir,
            slug,
            topic,
            include_drafts,
            template_dir,
            site_url,
        }) => {
            println!("Building content");
            tools::build_content(output_dir, slug, topic, include_drafts, template_dir, site_url)?;
        },
        None => run_interactive_cli()?,
    }

    Ok(())
}
