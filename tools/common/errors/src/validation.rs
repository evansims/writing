use crate::{Result, WritingError};

/// Extension trait for Option that provides standard validation methods
///
/// This trait extends the Option type with methods for standardized validation,
/// making it easier to validate optional values and convert them to Results.
///
/// # Example
///
/// ```rust
/// use common_errors::{Result, WritingError, OptionValidationExt};
///
/// fn process_name(name: Option<String>) -> Result<String> {
///     // Validate that the name is provided
///     let name = name.validate_required("Name is required")?;
///     
///     // Process the name
///     Ok(name.to_uppercase())
/// }
/// ```
pub trait OptionValidationExt<T> {
    /// Validate that an Option has a value, returning a ValidationError if None
    ///
    /// # Parameters
    ///
    /// * `error_message` - The error message to use if the Option is None
    ///
    /// # Returns
    ///
    /// The inner value if Some, or a ValidationError if None
    ///
    /// # Example
    ///
    /// ```rust
    /// use common_errors::{Result, OptionValidationExt};
    ///
    /// fn get_required_value(value: Option<String>) -> Result<String> {
    ///     value.validate_required("Value is required")
    /// }
    /// ```
    fn validate_required(self, error_message: &str) -> Result<T>;
    
    /// Validate that an Option has a value, executing a closure to create 
    /// a WritingError if None
    ///
    /// # Parameters
    ///
    /// * `error_fn` - A function that returns a WritingError if the Option is None
    ///
    /// # Returns
    ///
    /// The inner value if Some, or a custom WritingError if None
    ///
    /// # Example
    ///
    /// ```rust
    /// use common_errors::{Result, WritingError, OptionValidationExt};
    ///
    /// fn get_content_id(id: Option<String>) -> Result<String> {
    ///     id.validate_with(|| WritingError::content_not_found("Content ID not found"))
    /// }
    /// ```
    fn validate_with(self, error_fn: impl FnOnce() -> WritingError) -> Result<T>;
}

impl<T> OptionValidationExt<T> for Option<T> {
    fn validate_required(self, error_message: &str) -> Result<T> {
        match self {
            Some(value) => Ok(value),
            None => Err(WritingError::validation_error(error_message))
        }
    }
    
    fn validate_with(self, error_fn: impl FnOnce() -> WritingError) -> Result<T> {
        match self {
            Some(value) => Ok(value),
            None => Err(error_fn())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_validate_required() {
        let some_value: Option<i32> = Some(42);
        let result = some_value.validate_required("Value is required");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42);
        
        let none_value: Option<i32> = None;
        let result = none_value.validate_required("Value is required");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(format!("{}", err).contains("Value is required"));
    }
    
    #[test]
    fn test_validate_with() {
        let some_value: Option<String> = Some("test".to_string());
        let result = some_value.validate_with(|| WritingError::validation_error("Custom error"));
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "test");
        
        let none_value: Option<String> = None;
        let result = none_value.validate_with(|| WritingError::validation_error("Custom error"));
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(format!("{}", err).contains("Custom error"));
    }
} 