//! # Common Macros for Trait Implementations
//!
//! This module provides macros for common trait implementations to reduce boilerplate code.

/// Implement the `Default` trait for option structs with builder pattern.
///
/// This macro simplifies the implementation of the `Default` trait for option
/// structs that are commonly used in the codebase.
///
/// # Examples
///
/// ```
/// use common_macros::impl_default_options;
///
/// pub struct BuildOptions {
///     pub output_dir: Option<String>,
///     pub slug: Option<String>,
///     pub topic: Option<String>,
///     pub include_drafts: bool,
/// }
///
/// impl_default_options! {
///     BuildOptions {
///         output_dir: Option<String> = None,
///         slug: Option<String> = None,
///         topic: Option<String> = None,
///         include_drafts: bool = false,
///     }
/// }
/// ```
#[macro_export]
macro_rules! impl_default_options {
    ($struct_name:ident {
        $($field:ident: $type:ty = $default:expr),* $(,)?
    }) => {
        impl Default for $struct_name {
            fn default() -> Self {
                Self {
                    $($field: $default),*
                }
            }
        }

        impl $struct_name {
            /// Create a new instance with default values
            pub fn new() -> Self {
                Self::default()
            }

            /// Create a builder for this options struct
            pub fn builder() -> $struct_name {
                Self::default()
            }

            $(
                /// Set the value for this field
                pub fn $field(mut self, value: $type) -> Self {
                    self.$field = value;
                    self
                }
            )*
        }
    };
}

/// Implement common debug logging methods for a struct
///
/// This macro adds debug logging methods to a struct to simplify
/// tracing and debugging.
///
/// # Examples
///
/// ```
/// use common_macros::impl_debug_logging;
///
/// pub struct BuildProcess {
///     name: String,
/// }
///
/// impl BuildProcess {
///     pub fn new(name: &str) -> Self {
///         Self { name: name.to_string() }
///     }
/// }
///
/// impl_debug_logging! { BuildProcess }
/// ```
#[macro_export]
macro_rules! impl_debug_logging {
    ($struct_name:ident) => {
        impl $struct_name {
            /// Log a debug message with the struct's context
            pub fn debug_log(&self, message: &str) {
                eprintln!("[{}] DEBUG: {}", stringify!($struct_name), message);
            }

            /// Log an info message with the struct's context
            pub fn info_log(&self, message: &str) {
                println!("[{}] INFO: {}", stringify!($struct_name), message);
            }

            /// Log an error message with the struct's context
            pub fn error_log(&self, message: &str) {
                eprintln!("[{}] ERROR: {}", stringify!($struct_name), message);
            }

            /// Log the current state of the struct for debugging
            pub fn debug_state(&self) {
                self.debug_log(&format!("Current state: {:?}", self));
            }
        }
    };
}

/// Implement conversion from a string to an enum
///
/// This macro simplifies the implementation of string-to-enum conversion
/// which is commonly used for command-line arguments and configuration.
///
/// # Examples
///
/// ```
/// use common_macros::impl_from_str_enum;
///
/// #[derive(Debug, PartialEq)]
/// pub enum BuildMode {
///     Development,
///     Production,
///     Test,
/// }
///
/// impl_from_str_enum! {
///     BuildMode {
///         "dev" | "development" => BuildMode::Development,
///         "prod" | "production" => BuildMode::Production,
///         "test" => BuildMode::Test,
///     }
/// }
/// ```
#[macro_export]
macro_rules! impl_from_str_enum {
    ($enum_name:ident {
        $($pattern:pat => $variant:expr),* $(,)?
    }) => {
        impl std::str::FromStr for $enum_name {
            type Err = String;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let lower = s.to_lowercase();
                match lower.as_str() {
                    $($pattern => Ok($variant),)*
                    _ => Err(format!("Invalid {} value: {}", stringify!($enum_name), s)),
                }
            }
        }

        impl $enum_name {
            /// Convert from a string to the enum, with a default value if invalid
            pub fn from_str_or_default(s: &str, default: $enum_name) -> $enum_name {
                s.parse::<$enum_name>().unwrap_or(default)
            }

            /// Get all valid string representations of the enum
            pub fn valid_values() -> Vec<&'static str> {
                vec![$($pattern),*]
            }
        }
    };
}