use content_move::OptionValidationExt;
use anyhow::Result;

#[cfg(test)]
mod move_options_validation_tests {
    use super::*;

    #[test]
    fn test_validate_required_with_some_value() {
        // Arrange
        let option = Some("test".to_string());

        // Act
        let result = option.validate_required("Value is required");

        // Assert
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "test".to_string());
    }

    #[test]
    fn test_validate_required_with_none() {
        // Arrange
        let option: Option<String> = None;

        // Act
        let result = option.validate_required("Value is required");

        // Assert
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Value is required");
    }

    #[test]
    fn test_multiple_validations() {
        // Arrange
        let value1 = Some("value1".to_string());
        let value2: Option<String> = None;

        // Act & Assert
        let result1 = value1.validate_required("Value1 is required");
        let result2 = value2.validate_required("Value2 is required");

        assert!(result1.is_ok());
        assert!(result2.is_err());
        assert_eq!(result1.unwrap(), "value1".to_string());
        assert_eq!(result2.unwrap_err().to_string(), "Value2 is required");
    }
}