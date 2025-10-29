/// "Assert json matches"
/// Asserts that the given JSON in the first argument matches the JSON matcher defined by the second argument.
/// Panics if the JSON does not match any expectations. Will print out each error encountered as well as the actual JSON encountered.
///
/// ```
/// use serde_json::json;
/// use json_matcher::{assert_jm, AnyMatcher};
///
/// let test_data = json!({"name": "John", "age": 30});
///
/// // exact match against json defined in-line
/// assert_jm!(test_data, { "name": "John", "age": 30 });
///
/// // can also use non-exact "matchers"
/// assert_jm!(test_data, { "name": "John", "age": AnyMatcher::not_null() })
/// ```
#[macro_export]
macro_rules! assert_jm {
    // Handle object syntax directly
    ($actual:expr, { $($json:tt)* }) => {{
        let actual = &$actual;
        let expectation = $crate::create_json_matcher!({ $($json)* });
        let errors = $crate::JsonMatcher::json_matches(&expectation, &actual);
        if !errors.is_empty() {
            let bullets = errors
                .into_iter()
                .map(|e| format!("  - {}", e))
                .collect::<Vec<String>>();
            let error_message = format!("\nJson matcher failed:\n{}", bullets.join("\n"));
            let actual_message = format!(
                "Actual:\n{}",
                serde_json::to_string_pretty(&actual).unwrap()
            );
            panic!("{}\n\n{}", error_message, actual_message);
        }
    }};

    // Handle array syntax directly
    ($actual:expr, [ $($json:tt)* ]) => {{
        let actual = &$actual;
        let expectation = $crate::create_json_matcher!([ $($json)* ]);
        let errors = $crate::JsonMatcher::json_matches(&expectation, &actual);
        if !errors.is_empty() {
            let bullets = errors
                .into_iter()
                .map(|e| format!("  - {}", e))
                .collect::<Vec<String>>();
            let error_message = format!("\nJson matcher failed:\n{}", bullets.join("\n"));
            let actual_message = format!(
                "Actual:\n{}",
                serde_json::to_string_pretty(&actual).unwrap()
            );
            panic!("{}\n\n{}", error_message, actual_message);
        }
    }};

    // Handle literals directly
    ($actual:expr, $literal:literal) => {{
        let actual = &$actual;
        let expectation = $crate::create_json_matcher!($literal);
        let errors = $crate::JsonMatcher::json_matches(&expectation, &actual);
        if !errors.is_empty() {
            let bullets = errors
                .into_iter()
                .map(|e| format!("  - {}", e))
                .collect::<Vec<String>>();
            let error_message = format!("\nJson matcher failed:\n{}", bullets.join("\n"));
            let actual_message = format!(
                "Actual:\n{}",
                serde_json::to_string_pretty(&actual).unwrap()
            );
            panic!("{}\n\n{}", error_message, actual_message);
        }
    }};

    // Handle null
    ($actual:expr, null) => {{
        let actual = &$actual;
        let expectation = $crate::create_json_matcher!(null);
        let errors = $crate::JsonMatcher::json_matches(&expectation, &actual);
        if !errors.is_empty() {
            let bullets = errors
                .into_iter()
                .map(|e| format!("  - {}", e))
                .collect::<Vec<String>>();
            let error_message = format!("\nJson matcher failed:\n{}", bullets.join("\n"));
            let actual_message = format!(
                "Actual:\n{}",
                serde_json::to_string_pretty(&actual).unwrap()
            );
            panic!("{}\n\n{}", error_message, actual_message);
        }
    }};

    // Handle true
    ($actual:expr, true) => {{
        let actual = &$actual;
        let expectation = $crate::create_json_matcher!(true);
        let errors = $crate::JsonMatcher::json_matches(&expectation, &actual);
        if !errors.is_empty() {
            let bullets = errors
                .into_iter()
                .map(|e| format!("  - {}", e))
                .collect::<Vec<String>>();
            let error_message = format!("\nJson matcher failed:\n{}", bullets.join("\n"));
            let actual_message = format!(
                "Actual:\n{}",
                serde_json::to_string_pretty(&actual).unwrap()
            );
            panic!("{}\n\n{}", error_message, actual_message);
        }
    }};

    // Handle false
    ($actual:expr, false) => {{
        let actual = &$actual;
        let expectation = $crate::create_json_matcher!(false);
        let errors = $crate::JsonMatcher::json_matches(&expectation, &actual);
        if !errors.is_empty() {
            let bullets = errors
                .into_iter()
                .map(|e| format!("  - {}", e))
                .collect::<Vec<String>>();
            let error_message = format!("\nJson matcher failed:\n{}", bullets.join("\n"));
            let actual_message = format!(
                "Actual:\n{}",
                serde_json::to_string_pretty(&actual).unwrap()
            );
            panic!("{}\n\n{}", error_message, actual_message);
        }
    }};

    // Original syntax - when passed an expression (must be last)
    ($actual:expr, $expectation:expr) => {{
        let actual = &$actual;
        let expectation = &$expectation;
        let errors = $crate::JsonMatcher::json_matches(expectation, &actual);
        if !errors.is_empty() {
            let bullets = errors
                .into_iter()
                .map(|e| format!("  - {}", e))
                .collect::<Vec<String>>();
            let error_message = format!("\nJson matcher failed:\n{}", bullets.join("\n"));
            let actual_message = format!(
                "Actual:\n{}",
                serde_json::to_string_pretty(&actual).unwrap()
            );
            panic!("{}\n\n{}", error_message, actual_message);
        }
    }};
}

/// Create a json matcher from JSON-like syntax with embedded matchers
///
/// ```
/// use json_matcher::{
///     create_json_matcher, BooleanMatcher, JsonMatcher, JsonMatcherError, JsonPath, JsonPathElement,
/// };
/// use serde_json::json;
///
/// let matcher = create_json_matcher!({
///     "name": "John",
///     "is_cool": BooleanMatcher::any()
/// });
///
/// let test_data = json!({
///     "name": "John",
///     "is_cool": "unknown"
/// });
///
/// assert_eq!(
///     matcher.json_matches(&test_data),
///     vec![JsonMatcherError {
///         path: JsonPath::from(vec![
///             JsonPathElement::Root,
///             JsonPathElement::Key("is_cool".to_string())
///         ]),
///         message: "Value is not a boolean".to_string()
///     }]
/// );
/// ```
#[macro_export]
macro_rules! create_json_matcher {
    // Handle null
    (null) => {
        $crate::NullMatcher::new()
    };

    // Handle booleans
    (true) => {
        $crate::BooleanMatcher::exact(true)
    };
    (false) => {
        $crate::BooleanMatcher::exact(false)
    };

    // Handle numbers (integers and floats)
    ($num:literal) => {{
        // We'll use serde_json::json! to parse the number and then convert
        let value = serde_json::json!($num);
        value
    }};

    // Handle strings
    ($string:literal) => {
        $crate::StringMatcher::new($string)
    };

    // Handle arrays
    ([ $($item:tt),* $(,)? ]) => {
        $crate::ArrayMatcher::new()
            $(.element($crate::create_json_matcher!($item)))*
    };

    // Handle objects
    ({ $($json:tt)* }) => {
        $crate::create_json_matcher!(@object {} $($json)*)
    };

    // Internal rules for parsing object fields
    // Handle empty object (no fields)
    (@object {$($out:tt)*}) => {
        $crate::ObjectMatcher::new() $($out)*
    };
    // Handle nested objects
    (@object {$($out:tt)*} $key:literal : { $($value:tt)* } , $($rest:tt)*) => {
        $crate::create_json_matcher!(@object {$($out)* .field($key, $crate::create_json_matcher!({ $($value)* }))} $($rest)*)
    };
    (@object {$($out:tt)*} $key:literal : { $($value:tt)* }) => {
        $crate::ObjectMatcher::new() $($out)* .field($key, $crate::create_json_matcher!({ $($value)* }))
    };
    // Handle arrays
    (@object {$($out:tt)*} $key:literal : [ $($value:tt)* ] , $($rest:tt)*) => {
        $crate::create_json_matcher!(@object {$($out)* .field($key, $crate::create_json_matcher!([ $($value)* ]))} $($rest)*)
    };
    (@object {$($out:tt)*} $key:literal : [ $($value:tt)* ]) => {
        $crate::ObjectMatcher::new() $($out)* .field($key, $crate::create_json_matcher!([ $($value)* ]))
    };
    // Handle null, true, false keywords (must come before literals)
    (@object {$($out:tt)*} $key:literal : null , $($rest:tt)*) => {
        $crate::create_json_matcher!(@object {$($out)* .field($key, $crate::create_json_matcher!(null))} $($rest)*)
    };
    (@object {$($out:tt)*} $key:literal : null) => {
        $crate::ObjectMatcher::new() $($out)* .field($key, $crate::create_json_matcher!(null))
    };
    (@object {$($out:tt)*} $key:literal : true , $($rest:tt)*) => {
        $crate::create_json_matcher!(@object {$($out)* .field($key, $crate::create_json_matcher!(true))} $($rest)*)
    };
    (@object {$($out:tt)*} $key:literal : true) => {
        $crate::ObjectMatcher::new() $($out)* .field($key, $crate::create_json_matcher!(true))
    };
    (@object {$($out:tt)*} $key:literal : false , $($rest:tt)*) => {
        $crate::create_json_matcher!(@object {$($out)* .field($key, $crate::create_json_matcher!(false))} $($rest)*)
    };
    (@object {$($out:tt)*} $key:literal : false) => {
        $crate::ObjectMatcher::new() $($out)* .field($key, $crate::create_json_matcher!(false))
    };
    // Handle literals
    (@object {$($out:tt)*} $key:literal : $value:literal , $($rest:tt)*) => {
        $crate::create_json_matcher!(@object {$($out)* .field($key, $crate::create_json_matcher!($value))} $($rest)*)
    };
    (@object {$($out:tt)*} $key:literal : $value:literal) => {
        $crate::ObjectMatcher::new() $($out)* .field($key, $crate::create_json_matcher!($value))
    };
    // Handle identifiers as keys with null, true, false
    (@object {$($out:tt)*} $key:ident : null , $($rest:tt)*) => {
        $crate::create_json_matcher!(@object {$($out)* .field(stringify!($key), $crate::create_json_matcher!(null))} $($rest)*)
    };
    (@object {$($out:tt)*} $key:ident : null) => {
        $crate::ObjectMatcher::new() $($out)* .field(stringify!($key), $crate::create_json_matcher!(null))
    };
    (@object {$($out:tt)*} $key:ident : true , $($rest:tt)*) => {
        $crate::create_json_matcher!(@object {$($out)* .field(stringify!($key), $crate::create_json_matcher!(true))} $($rest)*)
    };
    (@object {$($out:tt)*} $key:ident : true) => {
        $crate::ObjectMatcher::new() $($out)* .field(stringify!($key), $crate::create_json_matcher!(true))
    };
    (@object {$($out:tt)*} $key:ident : false , $($rest:tt)*) => {
        $crate::create_json_matcher!(@object {$($out)* .field(stringify!($key), $crate::create_json_matcher!(false))} $($rest)*)
    };
    (@object {$($out:tt)*} $key:ident : false) => {
        $crate::ObjectMatcher::new() $($out)* .field(stringify!($key), $crate::create_json_matcher!(false))
    };
    // Handle identifiers as keys with literal values
    (@object {$($out:tt)*} $key:ident : $value:literal , $($rest:tt)*) => {
        $crate::create_json_matcher!(@object {$($out)* .field(stringify!($key), $crate::create_json_matcher!($value))} $($rest)*)
    };
    (@object {$($out:tt)*} $key:ident : $value:literal) => {
        $crate::ObjectMatcher::new() $($out)* .field(stringify!($key), $crate::create_json_matcher!($value))
    };
    // Handle expressions (matchers, variables, etc.) - must come last as catch-all
    (@object {$($out:tt)*} $key:literal : $value:expr , $($rest:tt)*) => {
        $crate::create_json_matcher!(@object {$($out)* .field($key, $value)} $($rest)*)
    };
    (@object {$($out:tt)*} $key:literal : $value:expr) => {
        $crate::ObjectMatcher::new() $($out)* .field($key, $value)
    };
    (@object {$($out:tt)*} $key:ident : $value:expr , $($rest:tt)*) => {
        $crate::create_json_matcher!(@object {$($out)* .field(stringify!($key), $value)} $($rest)*)
    };
    (@object {$($out:tt)*} $key:ident : $value:expr) => {
        $crate::ObjectMatcher::new() $($out)* .field(stringify!($key), $value)
    };

    // Handle expressions (for matcher types) - this must come last
    ($expr:expr) => {
        $expr
    };
}

#[cfg(test)]
mod tests {
    use crate::{assert_jm, create_json_matcher, test::catch_string_panic};
    use crate::{AnyMatcher, JsonMatcher};
    use serde_json::json;

    // Mock UuidMatcher for testing
    struct UuidMatcher;

    impl UuidMatcher {
        fn new() -> Self {
            Self
        }
    }

    impl JsonMatcher for UuidMatcher {
        fn json_matches(&self, value: &serde_json::Value) -> Vec<crate::JsonMatcherError> {
            match value.as_str() {
                Some(s) if s.len() == 36 && s.chars().filter(|&c| c == '-').count() == 4 => vec![],
                Some(_) => vec![crate::JsonMatcherError::at_root(
                    "Expected valid UUID format",
                )],
                None => vec![crate::JsonMatcherError::at_root("Expected string for UUID")],
            }
        }
    }

    #[test]
    fn test_assert_jm_with_json_syntax_success() {
        let actual = json!({
            "name": "John",
            "age": 30,
            "active": true
        });

        // Should pass - exact match using direct JSON syntax
        assert_jm!(actual, {
            "name": "John",
            "age": 30,
            "active": true
        });
    }

    #[test]
    fn test_assert_jm_with_json_syntax_failure() {
        // Should not pass
        assert_eq!(
            catch_string_panic(|| assert_jm!(json!({
                "name": "John",
                "age": 30,
                "active": true
            }), {
                "name": "John",
                "age": 35,
                "active": true
            })),
            r#"
Json matcher failed:
  - $.age: Expected integer 35 but got 30

Actual:
{
  "name": "John",
  "age": 30,
  "active": true
}"#
        );
    }

    #[test]
    fn test_assert_jm_with_matcher_expression_success() {
        let actual = json!({
            "id": "550e8400-e29b-41d4-a716-446655440000",
            "name": "John"
        });

        // Should pass - using embedded matcher
        assert_jm!(actual, {
            "id": UuidMatcher::new(),
            "name": "John"
        });
    }

    #[test]
    fn test_assert_jm_with_matcher_expression_failure_on_nested_matched() {
        // Should not pass
        assert_eq!(
            catch_string_panic(|| assert_jm!(json!({
                "id": "bloop",
                "name": "John"
            }), {
                "id": UuidMatcher::new(),
                "name": "John"
            })),
            r#"
Json matcher failed:
  - $.id: Expected valid UUID format

Actual:
{
  "id": "bloop",
  "name": "John"
}"#
        );
    }

    #[test]
    fn test_assert_jm_with_matcher_expression_failure_on_exact_match() {
        // Should not pass
        assert_eq!(
            catch_string_panic(|| assert_jm!(json!({
                "id": "550e8400-e29b-41d4-a716-446655440000",
                "name": "Jim"
            }), {
                "id": UuidMatcher::new(),
                "name": "John"
            })),
            r#"
Json matcher failed:
  - $.name: Expected string "John" but got "Jim"

Actual:
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "name": "Jim"
}"#
        );
    }

    #[test]
    fn test_assert_jm_with_matcher_expression_failure_on_both() {
        // Should not pass
        assert_eq!(
            catch_string_panic(|| assert_jm!(json!({
                "id": "bloop",
                "name": "Jim"
            }), {
                "id": UuidMatcher::new(),
                "name": "John"
            })),
            r#"
Json matcher failed:
  - $.id: Expected valid UUID format
  - $.name: Expected string "John" but got "Jim"

Actual:
{
  "id": "bloop",
  "name": "Jim"
}"#
        );
    }

    #[test]
    fn test_assert_jm_with_mixed_matchers() {
        let actual = json!({
            "id": "550e8400-e29b-41d4-a716-446655440000",
            "name": "John",
            "tags": ["admin", "user"],
            "metadata": {
                "created": "2023-01-01",
                "version": 1
            }
        });

        // Mix of exact values and matchers
        assert_jm!(actual, {
            "id": UuidMatcher::new(),
            "name": "John",
            "tags": ["admin", "user"],
            "metadata": {
                "created": AnyMatcher::new(),
                "version": 1
            }
        });
    }

    #[test]
    fn test_assert_jm_failure_message() {
        let actual = json!({
            "name": "Jane",
            "age": 25
        });

        let error_message = catch_string_panic(|| {
            assert_jm!(actual, {
                "name": "John",
                "age": 25
            });
        });

        assert!(error_message.contains("Json matcher failed"));
        assert!(error_message.contains("Expected string \"John\" but got \"Jane\""));
    }

    #[test]
    fn test_create_json_matcher_macro_directly() {
        let matcher = create_json_matcher!({
            "field1": "exact value",
            "field2": AnyMatcher::new()
        });

        let valid_json = json!({
            "field1": "exact value",
            "field2": "anything"
        });

        assert_eq!(matcher.json_matches(&valid_json), vec![]);
    }

    #[test]
    fn test_assert_jm_with_arrays() {
        let actual = json!({
            "items": [1, 2, 3],
            "names": ["Alice", "Bob"]
        });

        assert_jm!(actual, {
            "items": [1, 2, 3],
            "names": ["Alice", "Bob"]
        });
    }

    #[test]
    fn test_assert_jm_nested_objects() {
        let actual = json!({
            "user": {
                "profile": {
                    "name": "John",
                    "verified": true
                }
            }
        });

        assert_jm!(actual, {
            "user": {
                "profile": {
                    "name": "John",
                    "verified": true
                }
            }
        });
    }

    #[test]
    fn test_assert_jm_original_syntax_still_works() {
        let actual = json!({
            "value": "test"
        });

        // Original syntax with expression still works
        let matcher = create_json_matcher!({
            "value": "test"
        });
        assert_jm!(actual, matcher);

        // Can also use json! directly for exact matching
        assert_jm!(actual, json!({"value": "test"}));
    }

    #[test]
    fn test_assert_jm_direct_literals() {
        // Test direct string literal
        assert_jm!(json!("hello"), "hello");

        // Test direct number literal
        assert_jm!(json!(42), 42);

        // Test direct boolean literals
        assert_jm!(json!(true), true);
        assert_jm!(json!(false), false);

        // Test direct null
        assert_jm!(json!(null), null);

        // Test direct array
        assert_jm!(json!([1, 2, 3]), [1, 2, 3]);
    }

    #[test]
    fn test_empty_object() {
        // Test empty object matching
        assert_jm!(json!({}), {});

        // Test nested empty object
        assert_jm!(json!({"empty": {}}), {
            "empty": {}
        });

        // Test array with empty object
        assert_jm!(json!([{}]), [{}]);
    }
}
