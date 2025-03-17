# Common Utilities

This crate provides common utility functions used across all tools in the writing tools suite.

## Modules

The crate is organized into the following modules:

### Process Utilities (`process.rs`)

Utilities for running external processes and tools.

- `run_tool` - Run a tool by name
- `run_tool_command` - Run a tool command with string arguments
- `run_command` - Run a shell command
- `run_command_with_input` - Run a shell command with input

### UI Utilities (`ui.rs`)

Utilities for console output and user interface.

- `print_success` - Print a success message
- `print_error` - Print an error message
- `print_info` - Print an information message
- `print_warning` - Print a warning message
- `format_title` - Format a title for display
- `format_heading` - Format a heading for display
- `print_section` - Print a section heading
- `print_table_row` - Print a table row with consistent column widths
- `confirm` - Ask the user for confirmation (yes/no)
- `prompt` - Get user input with a prompt
- `with_spinner` - Show a spinner during a long-running operation
- `ProgressBar` - A simple progress bar implementation

### Pattern Utilities (`pattern.rs`)

Utilities for pattern matching and text processing.

- `matches_pattern` - Check if a string matches a pattern
- `extract_matches` - Extract all matches from a string using a regex pattern
- `extract_named_captures` - Extract named captures from a string using a regex pattern
- `is_valid_slug` - Check if a string is a valid slug
- `is_valid_email` - Check if a string is a valid email
- `is_valid_url` - Check if a string is a valid URL
- `extract_markdown_links` - Extract all markdown links from a string
- `extract_frontmatter` - Extract frontmatter from markdown content
- `split_frontmatter_and_body` - Split content into frontmatter and body

### Time Utilities (`time.rs`)

Utilities for working with dates and times.

- `format_timestamp` - Format a timestamp as a human-readable string
- `format_relative_time` - Format a timestamp as a human-readable relative time
- `parse_date` - Parse a date string in YYYY-MM-DD format
- `parse_datetime` - Parse a datetime string in various formats
- `current_timestamp` - Get the current timestamp
- `current_date_string` - Get the current date as a string in YYYY-MM-DD format
- `current_datetime_string` - Get the current datetime as a string in YYYY-MM-DD HH:MM:SS format
- `days_between` - Calculate the difference between two dates in days
- `is_future_date` - Check if a date is in the future
- `is_past_date` - Check if a date is in the past
- `add_days` - Add days to a date
- `start_of_month` - Get the start of the month for a given date
- `end_of_month` - Get the end of the month for a given date

## Usage

To use this crate, add it as a dependency in your `Cargo.toml`:

```toml
[dependencies]
common_utils = { path = "../common/utils" }
```

Then import the desired modules or functions:

```rust
use common_utils::ui::{print_success, print_error};
use common_utils::pattern::is_valid_slug;
use common_utils::time::format_timestamp;
```

For frequently used utilities, you can also use the re-exports from the crate root:

```rust
use common_utils::{print_success, print_error, matches_pattern, format_timestamp};
``` 