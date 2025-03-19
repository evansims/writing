#[cfg(test)]
mod integration_tests {
    use std::process::Command;
    use std::time::Duration;
    use std::thread;

    #[test]
    fn test_help_command() {
        // First build the binary to ensure it exists
        let build = Command::new("cargo")
            .args(["build", "--package", "content-validate"])
            .output()
            .expect("Failed to build command");

        assert!(build.status.success(), "Failed to build content-validate");

        // Give a short delay to ensure the binary is available
        thread::sleep(Duration::from_millis(100));

        // Then run the help command
        let output = Command::new("cargo")
            .args(["run", "--package", "content-validate", "--", "--help"])
            .output()
            .expect("Failed to execute command");

        assert!(output.status.success(), "Command failed: {}", String::from_utf8_lossy(&output.stderr));

        let stdout = String::from_utf8_lossy(&output.stdout);
        let stdout_lower = stdout.to_lowercase();
        println!("Command output: {}", stdout);

        assert!(stdout_lower.contains("usage"), "Missing usage information");
        assert!(stdout_lower.contains("options"), "Missing options information");
    }
}