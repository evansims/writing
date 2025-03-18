//! Test helper macros
//!
//! This module provides macros for common test patterns.

/// Create a test fixture and pass it to the provided code block
///
/// # Example
///
/// ```
/// use common_test_utils::with_test_fixture;
///
/// #[test]
/// fn test_with_fixture() {
///     with_test_fixture!(fixture => {
///         // Test code using fixture
///         let content_path = fixture.create_content("blog", "test-post", "Test Post", false).unwrap();
///         assert!(content_path.exists());
///     });
/// }
/// ```
#[macro_export]
macro_rules! with_test_fixture {
    ($fixture:ident => $block:block) => {
        let $fixture = $crate::TestFixture::new().unwrap();
        $block
        // Fixture will be automatically cleaned up when it goes out of scope
    };
}

/// Create a mock and pass it to the provided code block
///
/// # Example
///
/// ```
/// use common_test_utils::with_mock;
///
/// #[test]
/// fn test_with_mock() {
///     with_mock!(MockFileSystem, mock_fs => {
///         // Setup expectations
///         mock_fs.expect_file_exists()
///             .returning(|_| Ok(true));
///
///         // Test code using mock_fs
///     });
/// }
/// ```
#[macro_export]
macro_rules! with_mock {
    ($mock_type:ident, $mock_var:ident => $block:block) => {
        let mut $mock_var = $crate::mocks::$mock_type::new();
        $block
    };
}

/// Create multiple mocks and pass them to the provided code block
///
/// # Example
///
/// ```
/// use common_test_utils::with_mocks;
///
/// #[test]
/// fn test_with_mocks() {
///     with_mocks!([
///         MockFileSystem, mock_fs,
///         MockConfigLoader, mock_config
///     ] => {
///         // Setup expectations
///         mock_fs.expect_file_exists()
///             .returning(|_| Ok(true));
///
///         mock_config.expect_load_config()
///             .returning(|| Ok(Config::default()));
///
///         // Test code using mocks
///     });
/// }
/// ```
#[macro_export]
macro_rules! with_mocks {
    ([$($mock_type:ident, $mock_var:ident),*] => $block:block) => {
        $(
            let mut $mock_var = $crate::mocks::$mock_type::new();
        )*
        $block
    };
}

/// Assert that a string contains a substring
///
/// # Example
///
/// ```
/// use common_test_utils::assert_contains;
///
/// #[test]
/// fn test_with_assert_contains() {
///     let result = "Hello, world!";
///     assert_contains!(result, "world");
/// }
/// ```
#[macro_export]
macro_rules! assert_contains {
    ($haystack:expr, $needle:expr) => {
        assert!(
            $haystack.contains($needle),
            "Expected '{}' to contain '{}', but it didn't",
            $haystack, $needle
        );
    };
    ($haystack:expr, $needle:expr, $($arg:tt)+) => {
        assert!(
            $haystack.contains($needle),
            $($arg)+
        );
    };
}

/// Assert that a string does not contain a substring
///
/// # Example
///
/// ```
/// use common_test_utils::assert_not_contains;
///
/// #[test]
/// fn test_with_assert_not_contains() {
///     let result = "Hello, world!";
///     assert_not_contains!(result, "goodbye");
/// }
/// ```
#[macro_export]
macro_rules! assert_not_contains {
    ($haystack:expr, $needle:expr) => {
        assert!(
            !$haystack.contains($needle),
            "Expected '{}' to not contain '{}', but it did",
            $haystack, $needle
        );
    };
    ($haystack:expr, $needle:expr, $($arg:tt)+) => {
        assert!(
            !$haystack.contains($needle),
            $($arg)+
        );
    };
}

/// Assert that a result is Ok and contains a value matching a predicate
///
/// # Example
///
/// ```
/// use common_test_utils::assert_ok_matches;
///
/// #[test]
/// fn test_with_assert_ok_matches() {
///     let result: Result<i32, &str> = Ok(42);
///     assert_ok_matches!(result, |value| *value == 42);
/// }
/// ```
#[macro_export]
macro_rules! assert_ok_matches {
    ($result:expr, $predicate:expr) => {
        match $result {
            Ok(value) => {
                let predicate = $predicate;
                assert!(
                    predicate(&value),
                    "Result was Ok({:?}), but didn't match the predicate",
                    value
                );
            }
            Err(err) => {
                panic!("Expected Ok, but got Err({:?})", err);
            }
        }
    };
    ($result:expr, $predicate:expr, $($arg:tt)+) => {
        match $result {
            Ok(value) => {
                let predicate = $predicate;
                assert!(
                    predicate(&value),
                    $($arg)+
                );
            }
            Err(err) => {
                panic!("Expected Ok, but got Err({:?}): {}", err, format!($($arg)+));
            }
        }
    };
}

/// Assert that a result is Err and contains an error matching a predicate
///
/// # Example
///
/// ```
/// use common_test_utils::assert_err_matches;
///
/// #[test]
/// fn test_with_assert_err_matches() {
///     let result: Result<i32, &str> = Err("invalid input");
///     assert_err_matches!(result, |err| err.contains("invalid"));
/// }
/// ```
#[macro_export]
macro_rules! assert_err_matches {
    ($result:expr, $predicate:expr) => {
        match $result {
            Ok(value) => {
                panic!("Expected Err, but got Ok({:?})", value);
            }
            Err(err) => {
                let predicate = $predicate;
                assert!(
                    predicate(&err),
                    "Result was Err({:?}), but didn't match the predicate",
                    err
                );
            }
        }
    };
    ($result:expr, $predicate:expr, $($arg:tt)+) => {
        match $result {
            Ok(value) => {
                panic!("Expected Err, but got Ok({:?}): {}", value, format!($($arg)+));
            }
            Err(err) => {
                let predicate = $predicate;
                assert!(
                    predicate(&err),
                    $($arg)+
                );
            }
        }
    };
}

/// Test a property with a range of inputs
///
/// # Example
///
/// ```
/// use common_test_utils::test_property;
///
/// #[test]
/// fn test_with_property() {
///     test_property!(
///         inputs = [1, 2, 3, 4, 5],
///         property = |x| x * 2 > x,
///         description = "doubling a number should result in a larger number"
///     );
/// }
/// ```
#[macro_export]
macro_rules! test_property {
    (inputs = $inputs:expr, property = $property:expr, description = $description:expr) => {
        let inputs = $inputs;
        let property = $property;

        for input in inputs {
            assert!(
                property(input),
                "Property '{}' did not hold for input: {:?}",
                $description,
                input
            );
        }
    };
}

/// Run a test with a variety of mock return values
///
/// # Example
///
/// ```
/// use common_test_utils::test_with_mock_returns;
///
/// #[test]
/// fn test_with_various_returns() {
///     test_with_mock_returns!(
///         mock_type = MockFileSystem,
///         mock_var = mock_fs,
///         method = expect_file_exists,
///         returns = [
///             Ok(true),
///             Ok(false),
///             Err(WritingError::not_found("file not found"))
///         ],
///         test_fn = |mock_fs| {
///             let sut = System::new(Box::new(mock_fs));
///             sut.do_something()
///         },
///         assertions = [
///             |result| assert!(result.is_ok()),
///             |result| assert!(result.is_err()),
///             |result| assert_err_matches!(result, |e| e.to_string().contains("not found"))
///         ]
///     );
/// }
/// ```
#[macro_export]
macro_rules! test_with_mock_returns {
    (
        mock_type = $mock_type:ident,
        mock_var = $mock_var:ident,
        method = $method:ident,
        returns = [$($return_value:expr),*],
        test_fn = $test_fn:expr,
        assertions = [$($assertion:expr),*]
    ) => {
        let return_values = [$($return_value),*];
        let assertions = [$($assertion),*];

        assert_eq!(
            return_values.len(),
            assertions.len(),
            "Number of return values must match number of assertions"
        );

        for (i, (return_value, assertion)) in return_values.into_iter().zip(assertions.into_iter()).enumerate() {
            let mut $mock_var = $crate::mocks::$mock_type::new();
            $mock_var.$method()
                .returning(move |_| return_value.clone());

            let result = $test_fn($mock_var);
            assertion(result);
        }
    };
}

/// Time the execution of a code block
///
/// # Example
///
/// ```
/// use common_test_utils::time_execution;
///
/// #[test]
/// fn test_performance() {
///     let duration = time_execution!({
///         // Code to time
///         std::thread::sleep(std::time::Duration::from_millis(10));
///     });
///
///     assert!(duration.as_millis() >= 10, "Execution took: {:?}", duration);
/// }
/// ```
#[macro_export]
macro_rules! time_execution {
    ($block:block) => {{
        let start = std::time::Instant::now();
        $block
        let duration = std::time::Instant::now() - start;
        duration
    }};
}