//! Resource cleanup utilities for ensuring proper handling of file resources.
//!
//! This module provides utilities for ensuring that file handles and other resources
//! are properly closed and cleaned up, leveraging Rust's RAII pattern.

use std::fs::{File, OpenOptions};
use std::io::{self, BufReader, BufWriter, Read, Write};
use std::path::{Path, PathBuf};
use common_errors::{Result, ErrorContext, IoResultExt, WritingError};

/// A wrapper around `File` that ensures the file is properly closed when it goes out of scope.
///
/// This struct implements the `Drop` trait to ensure that any resources associated with
/// the file are properly cleaned up when the struct is dropped.
///
/// # Examples
///
/// ```
/// use common_fs::cleanup::SafeFile;
/// use std::io::Read;
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let mut file = SafeFile::open("Cargo.toml")?;
/// let mut contents = String::new();
/// file.read_to_string(&mut contents)?;
/// // File will be automatically closed when `file` goes out of scope
/// # Ok(())
/// # }
/// ```
pub struct SafeFile {
    file: File,
    path: PathBuf,
}

impl SafeFile {
    /// Opens a file in read-only mode.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the file to open
    ///
    /// # Returns
    ///
    /// A `Result` containing the `SafeFile` if successful, or an error if the file could not be opened.
    ///
    /// # Examples
    ///
    /// ```
    /// use common_fs::cleanup::SafeFile;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let file = SafeFile::open("Cargo.toml")?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path_buf = path.as_ref().to_path_buf();
        let file = File::open(&path_buf)
            .with_enhanced_context(|| {
                ErrorContext::new("open file")
                    .with_file(&path_buf)
                    .with_details("Unable to open file for reading")
            })?;

        Ok(SafeFile {
            file,
            path: path_buf,
        })
    }

    /// Opens a file in write-only mode, creating it if it doesn't exist or truncating it if it does.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the file to create or truncate
    ///
    /// # Returns
    ///
    /// A `Result` containing the `SafeFile` if successful, or an error if the file could not be created.
    ///
    /// # Examples
    ///
    /// ```
    /// use common_fs::cleanup::SafeFile;
    /// use std::io::Write;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut file = SafeFile::create("example.txt")?;
    /// file.write_all(b"Hello, world!")?;
    /// # std::fs::remove_file("example.txt").ok();
    /// # Ok(())
    /// # }
    /// ```
    pub fn create<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path_buf = path.as_ref().to_path_buf();
        let file = File::create(&path_buf)
            .with_enhanced_context(|| {
                ErrorContext::new("create file")
                    .with_file(&path_buf)
                    .with_details("Unable to create file for writing")
            })?;

        Ok(SafeFile {
            file,
            path: path_buf,
        })
    }

    /// Opens a file with custom options.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the file to open
    /// * `options` - The options to use when opening the file
    ///
    /// # Returns
    ///
    /// A `Result` containing the `SafeFile` if successful, or an error if the file could not be opened.
    ///
    /// # Examples
    ///
    /// ```
    /// use common_fs::cleanup::SafeFile;
    /// use std::fs::OpenOptions;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut options = OpenOptions::new();
    /// options.read(true).write(true).create(true);
    /// let file = SafeFile::with_options("example.txt", options)?;
    /// # std::fs::remove_file("example.txt").ok();
    /// # Ok(())
    /// # }
    /// ```
    pub fn with_options<P: AsRef<Path>>(path: P, options: OpenOptions) -> Result<Self> {
        let path_buf = path.as_ref().to_path_buf();
        let file = options.open(&path_buf)
            .with_enhanced_context(|| {
                ErrorContext::new("open file with options")
                    .with_file(&path_buf)
                    .with_details("Unable to open file with specified options")
            })?;

        Ok(SafeFile {
            file,
            path: path_buf,
        })
    }

    /// Returns a reference to the underlying `File`.
    ///
    /// # Returns
    ///
    /// A reference to the underlying `File`.
    pub fn as_file(&self) -> &File {
        &self.file
    }

    /// Returns a mutable reference to the underlying `File`.
    ///
    /// # Returns
    ///
    /// A mutable reference to the underlying `File`.
    pub fn as_file_mut(&mut self) -> &mut File {
        &mut self.file
    }

    /// Returns the path of the file.
    ///
    /// # Returns
    ///
    /// A reference to the path of the file.
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// The underlying `File`.
    ///
    /// # Note
    ///
    /// This method should be used with caution, as it transfers responsibility for
    /// closing the file to the caller.
    pub fn into_file(mut self) -> File {
        // Take ownership of the file and replace it with a dummy file
        // This is a workaround for the Drop trait implementation
        let dummy_file = OpenOptions::new()
            .read(true)
            .open("/dev/null")
            .unwrap_or_else(|_| {
                // Fallback for non-Unix systems
                OpenOptions::new()
                    .read(true)
                    .open(&self.path)
                    .unwrap_or_else(|_| panic!("Failed to open dummy file"))
            });

        std::mem::replace(&mut self.file, dummy_file)
    }

    /// Creates a buffered reader from the file.
    ///
    /// # Returns
    ///
    /// A `BufReader` wrapping a reference to the file.
    ///
    /// # Examples
    ///
    /// ```
    /// use common_fs::cleanup::SafeFile;
    /// use std::io::BufRead;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let file = SafeFile::open("Cargo.toml")?;
    /// let reader = file.buf_reader();
    /// for line in reader.lines() {
    ///     println!("{}", line?);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn buf_reader(&self) -> BufReader<&File> {
        BufReader::new(&self.file)
    }

    /// Creates a buffered writer from the file.
    ///
    /// # Returns
    ///
    /// A `BufWriter` wrapping a reference to the file.
    ///
    /// # Examples
    ///
    /// ```
    /// use common_fs::cleanup::SafeFile;
    /// use std::io::Write;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let file = SafeFile::create("example.txt")?;
    /// let mut writer = file.buf_writer();
    /// writeln!(writer, "Hello, world!")?;
    /// # std::fs::remove_file("example.txt").ok();
    /// # Ok(())
    /// # }
    /// ```
    pub fn buf_writer(&self) -> BufWriter<&File> {
        BufWriter::new(&self.file)
    }
}

impl Read for SafeFile {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.file.read(buf)
    }
}

impl Write for SafeFile {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.file.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.file.flush()
    }
}

impl Drop for SafeFile {
    fn drop(&mut self) {
        // The file will be automatically closed when it's dropped,
        // but we could add additional cleanup logic here if needed.
        // For example, logging that the file was closed.
    }
}

/// A utility for safely reading a file to a string.
///
/// This function ensures that the file handle is properly closed after reading.
///
/// # Arguments
///
/// * `path` - The path to the file to read
///
/// # Returns
///
/// A `Result` containing the contents of the file as a string if successful,
/// or an error if the file could not be read.
///
/// # Examples
///
/// ```
/// use common_fs::cleanup::read_to_string;
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let contents = read_to_string("Cargo.toml")?;
/// println!("{}", contents);
/// # Ok(())
/// # }
/// ```
pub fn read_to_string<P: AsRef<Path>>(path: P) -> Result<String> {
    let mut file = SafeFile::open(&path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .with_enhanced_context(|| {
            ErrorContext::new("read file to string")
                .with_file(path.as_ref())
                .with_details("Unable to read file contents")
        })?;

    Ok(contents)
}

/// A utility for safely writing a string to a file.
///
/// This function ensures that the file handle is properly closed after writing.
///
/// # Arguments
///
/// * `path` - The path to the file to write
/// * `contents` - The string to write to the file
///
/// # Returns
///
/// A `Result` indicating success or failure.
///
/// # Examples
///
/// ```
/// use common_fs::cleanup::write_string;
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// write_string("example.txt", "Hello, world!")?;
/// # std::fs::remove_file("example.txt").ok();
/// # Ok(())
/// # }
/// ```
pub fn write_string<P: AsRef<Path>>(path: P, contents: &str) -> Result<()> {
    let mut file = SafeFile::create(&path)?;
    file.write_all(contents.as_bytes())
        .with_enhanced_context(|| {
            ErrorContext::new("write string to file")
                .with_file(path.as_ref())
                .with_details("Unable to write contents to file")
        })?;

    Ok(())
}

/// A utility for safely appending a string to a file.
///
/// This function ensures that the file handle is properly closed after writing.
///
/// # Arguments
///
/// * `path` - The path to the file to append to
/// * `contents` - The string to append to the file
///
/// # Returns
///
/// A `Result` indicating success or failure.
///
/// # Examples
///
/// ```
/// use common_fs::cleanup::{write_string, append_string};
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// write_string("example.txt", "Hello, ")?;
/// append_string("example.txt", "world!").unwrap();
/// # std::fs::remove_file("example.txt").ok();
/// # Ok(())
/// # }
/// ```
pub fn append_string<P: AsRef<Path>>(path: P, contents: &str) -> Result<()> {
    let mut options = OpenOptions::new();
    options.write(true).append(true).create(true);

    let mut file = SafeFile::with_options(&path, options)?;
    file.write_all(contents.as_bytes())
        .with_enhanced_context(|| {
            ErrorContext::new("append string to file")
                .with_file(path.as_ref())
                .with_details("Unable to append contents to file")
        })?;

    Ok(())
}

/// A utility for safely copying a file.
///
/// This function ensures that file handles are properly closed after copying.
///
/// # Arguments
///
/// * `from` - The path to the source file
/// * `to` - The path to the destination file
///
/// # Returns
///
/// A `Result` indicating success or failure.
///
/// # Examples
///
/// ```
/// use common_fs::cleanup::{write_string, copy_file};
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// write_string("source.txt", "Hello, world!").unwrap();
/// copy_file("source.txt", "destination.txt")?;
/// # std::fs::remove_file("source.txt").ok();
/// # std::fs::remove_file("destination.txt").ok();
/// # Ok(())
/// # }
/// ```
pub fn copy_file<P: AsRef<Path>, Q: AsRef<Path>>(from: P, to: Q) -> Result<()> {
    let mut source = SafeFile::open(&from)?;
    let mut dest = SafeFile::create(&to)?;

    // Use std::io::copy which is more functional than manual buffer management
    std::io::copy(&mut source, &mut dest)
        .with_enhanced_context(|| {
            ErrorContext::new("copy file")
                .with_file(from.as_ref())
                .with_details(format!("Unable to copy file to {}", to.as_ref().display()))
        })?;

    Ok(())
}

/// Copy a file to a new location using the standard library (no fs_extra dependency)
pub fn copy_file_std<P: AsRef<Path>, Q: AsRef<Path>>(from: P, to: Q) -> Result<()> {
    let from = from.as_ref();
    let to = to.as_ref();

    if let Some(parent) = to.parent() {
        if !parent.exists() {
            std::fs::create_dir_all(parent)
                .with_enhanced_context(|| {
                    ErrorContext::new("create directory")
                        .with_file(parent)
                        .with_details("Unable to create directory and parent directories")
                })?;
        }
    }

    if from.is_file() {
        std::fs::copy(from, to)
            .map(|_| ())
            .with_enhanced_context(|| {
                ErrorContext::new("copy file")
                    .with_file(from)
                    .with_details(format!("Failed to copy file to {}", to.display()))
            })
    } else {
        Err(WritingError::other(format!(
            "Source path is not a file: {}",
            from.display()
        )))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn test_safe_file_open() {
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path();

        let file = SafeFile::open(path).unwrap();
        assert_eq!(file.path(), path);
    }

    #[test]
    fn test_safe_file_create() {
        let temp_dir = tempfile::tempdir().unwrap();
        let path = temp_dir.path().join("test.txt");

        let mut file = SafeFile::create(&path).unwrap();
        file.write_all(b"Hello, world!").unwrap();

        // File should be automatically closed when it goes out of scope
        drop(file);

        // Verify the file was written correctly
        let contents = std::fs::read_to_string(&path).unwrap();
        assert_eq!(contents, "Hello, world!");
    }

    #[test]
    fn test_read_to_string() {
        let mut temp_file = NamedTempFile::new().unwrap();
        std::io::Write::write_all(&mut temp_file, b"Hello, world!").unwrap();

        let contents = read_to_string(temp_file.path()).unwrap();
        assert_eq!(contents, "Hello, world!");
    }

    #[test]
    fn test_write_string() {
        let temp_dir = tempfile::tempdir().unwrap();
        let path = temp_dir.path().join("test.txt");

        write_string(&path, "Hello, world!").unwrap();

        let contents = std::fs::read_to_string(&path).unwrap();
        assert_eq!(contents, "Hello, world!");
    }

    #[test]
    fn test_append_string() {
        let temp_dir = tempfile::tempdir().unwrap();
        let path = temp_dir.path().join("test.txt");

        write_string(&path, "Hello, ").unwrap();
        append_string(&path, "world!").unwrap();

        let contents = std::fs::read_to_string(&path).unwrap();
        assert_eq!(contents, "Hello, world!");
    }

    #[test]
    fn test_copy_file() {
        let temp_dir = tempfile::tempdir().unwrap();
        let source_path = temp_dir.path().join("source.txt");
        let dest_path = temp_dir.path().join("dest.txt");

        write_string(&source_path, "Hello, world!").unwrap();
        copy_file(&source_path, &dest_path).unwrap();

        let contents = std::fs::read_to_string(&dest_path).unwrap();
        assert_eq!(contents, "Hello, world!");
    }
}