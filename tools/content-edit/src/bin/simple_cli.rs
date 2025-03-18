use content_edit::cli_utils::{parse_args, run_command};
use std::process;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let command = parse_args(args);
    let exit_code = run_command(command);
    process::exit(exit_code);
}