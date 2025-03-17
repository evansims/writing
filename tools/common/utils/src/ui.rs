//! # UI Utilities
//! 
//! This module provides utilities for console output and user interface.

use colored::*;

/// Print a success message
pub fn print_success(message: &str) {
    println!("{}", message.green());
}

/// Print an error message
pub fn print_error(message: &str) {
    eprintln!("{}", message.red().bold());
}

/// Print an information message
pub fn print_info(message: &str) {
    println!("{}", message.blue());
}

/// Print a warning message
pub fn print_warning(message: &str) {
    println!("{}", message.yellow());
}

/// Format a title for display
pub fn format_title(title: &str) -> String {
    format!("{}", title.bold())
}

/// Format a heading for display
pub fn format_heading(heading: &str) -> String {
    format!("{}", heading.cyan().bold())
}

/// Print a section heading
pub fn print_section(title: &str) {
    println!("\n{}", format_heading(title));
    println!("{}", "=".repeat(title.len()));
}

/// Print a table row with consistent column widths
pub fn print_table_row(columns: &[(&str, usize)]) {
    let mut row = String::new();
    
    for (text, width) in columns {
        let formatted = format!("{:<width$}", text, width = width);
        row.push_str(&formatted);
        row.push_str("  ");
    }
    
    println!("{}", row);
}

/// Ask the user for confirmation (yes/no)
pub fn confirm(prompt: &str) -> bool {
    loop {
        print!("{} [y/n]: ", prompt.cyan());
        std::io::stdout().flush().unwrap();
        
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        
        match input.trim().to_lowercase().as_str() {
            "y" | "yes" => return true,
            "n" | "no" => return false,
            _ => println!("Please enter 'y' or 'n'"),
        }
    }
}

/// Get user input with a prompt
pub fn prompt(message: &str) -> String {
    print!("{}: ", message.cyan());
    std::io::stdout().flush().unwrap();
    
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    
    input.trim().to_string()
}

/// Show a spinner during a long-running operation
pub fn with_spinner<F, T>(message: &str, operation: F) -> T
where
    F: FnOnce() -> T,
{
    use std::{thread, time::Duration};
    use std::sync::{Arc, Mutex};
    use std::io::Write;
    
    let spinner_chars = vec!['⠋', '⠙', '⠹', '⠸', '⠼', '⠴', '⠦', '⠧', '⠇', '⠏'];
    let result = Arc::new(Mutex::new(None));
    let result_clone = result.clone();
    
    // Create a flag to signal the spinner thread to stop
    let running = Arc::new(Mutex::new(true));
    let running_clone = running.clone();
    
    // Spawn the spinner thread
    let spinner_handle = thread::spawn(move || {
        let mut i = 0;
        while *running_clone.lock().unwrap() {
            print!("\r{} {} ", spinner_chars[i % spinner_chars.len()], message);
            std::io::stdout().flush().unwrap();
            thread::sleep(Duration::from_millis(100));
            i += 1;
        }
        print!("\r");
        std::io::stdout().flush().unwrap();
    });
    
    // Run the operation
    let op_result = operation();
    
    // Signal the spinner thread to stop
    *running.lock().unwrap() = false;
    
    // Store the result
    *result_clone.lock().unwrap() = Some(op_result);
    
    // Wait for the spinner thread to finish
    spinner_handle.join().unwrap();
    
    // Return the result
    result.lock().unwrap().take().unwrap()
}

// Implement a simple progress bar
pub struct ProgressBar {
    total: usize,
    current: usize,
    width: usize,
}

impl ProgressBar {
    pub fn new(total: usize) -> Self {
        Self {
            total,
            current: 0,
            width: 50,
        }
    }
    
    pub fn with_width(total: usize, width: usize) -> Self {
        Self {
            total,
            current: 0,
            width,
        }
    }
    
    pub fn update(&mut self, current: usize) {
        self.current = current;
        self.draw();
    }
    
    pub fn increment(&mut self) {
        self.current += 1;
        self.draw();
    }
    
    fn draw(&self) {
        let percent = (self.current as f64 / self.total as f64) * 100.0;
        let filled_width = (self.width as f64 * (self.current as f64 / self.total as f64)) as usize;
        
        let bar = format!(
            "[{}{}] {:.1}% ({}/{})",
            "=".repeat(filled_width),
            " ".repeat(self.width - filled_width),
            percent,
            self.current,
            self.total
        );
        
        print!("\r{}", bar);
        std::io::stdout().flush().unwrap();
        
        if self.current == self.total {
            println!();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_format_title() {
        let formatted = format_title("Test Title");
        assert!(formatted.contains("Test Title"));
    }
    
    #[test]
    fn test_format_heading() {
        let formatted = format_heading("Test Heading");
        assert!(formatted.contains("Test Heading"));
    }
    
    #[test]
    fn test_progress_bar() {
        let mut progress = ProgressBar::new(10);
        progress.update(5);
        assert_eq!(progress.current, 5);
        
        progress.increment();
        assert_eq!(progress.current, 6);
    }
} 