use crate::{JsonMatcher, JsonMatcherError};

pub struct U16Matcher {
    allow_strings: bool,
}

impl Default for U16Matcher {
    fn default() -> Self {
        Self::new()
    }
}

impl U16Matcher {
    pub fn new() -> Self {
        Self {
            allow_strings: false,
        }
    }

    pub fn new_allow_strings() -> Self {
        Self {
            allow_strings: true,
        }
    }
}

impl JsonMatcher for U16Matcher {
    fn json_matches(&self, value: &serde_json::Value) -> Vec<JsonMatcherError> {
        match self.allow_strings {
            true => match value.as_str() {
                Some(s) if s.parse::<u16>().is_ok() => vec![],
                Some(_) => vec![JsonMatcherError::at_root("Expected number fitting u16")],
                None => vec![JsonMatcherError::at_root("Expected string fitting u16")],
            },
            false => match value.as_i64() {
                Some(s) if (0..=65535).contains(&s) => vec![],
                Some(_) => vec![JsonMatcherError::at_root("Integer out of bounds for u16")],
                None => vec![JsonMatcherError::at_root("Expected number fitting u16")],
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::assert_jm;
    use serde_json::Value;

    use super::*;

    #[test]
    fn test_u16_matcher_valid_values() {
        let get_matcher = || U16Matcher::new();

        // Test valid u16 values
        assert_jm!(Value::Number(0.into()), get_matcher());
        assert_jm!(Value::Number(1.into()), get_matcher());
        assert_jm!(Value::Number(65535.into()), get_matcher());
        assert_jm!(Value::Number(32768.into()), get_matcher());
        assert_jm!(Value::Number(100.into()), get_matcher());
    }

    #[test]
    fn test_u16_matcher_out_of_bounds() {
        let get_matcher = || U16Matcher::new();

        // Test negative values
        assert_eq!(
            *std::panic::catch_unwind(|| { assert_jm!(Value::Number((-1).into()), get_matcher()) })
                .err()
                .unwrap()
                .downcast::<String>()
                .unwrap(),
            r#"
Json matcher failed:
  - $: Integer out of bounds for u16

Actual:
-1"#
        );

        // Test values above u16 max
        assert_eq!(
            *std::panic::catch_unwind(|| {
                assert_jm!(Value::Number(65536.into()), get_matcher())
            })
            .err()
            .unwrap()
            .downcast::<String>()
            .unwrap(),
            r#"
Json matcher failed:
  - $: Integer out of bounds for u16

Actual:
65536"#
        );

        assert_eq!(
            *std::panic::catch_unwind(|| {
                assert_jm!(Value::Number(100000.into()), get_matcher())
            })
            .err()
            .unwrap()
            .downcast::<String>()
            .unwrap(),
            r#"
Json matcher failed:
  - $: Integer out of bounds for u16

Actual:
100000"#
        );
    }

    #[test]
    fn test_u16_matcher_non_numeric_values() {
        let get_matcher = || U16Matcher::new();

        // Test string value
        assert_eq!(
            *std::panic::catch_unwind(|| {
                assert_jm!(Value::String("42".to_string()), get_matcher())
            })
            .err()
            .unwrap()
            .downcast::<String>()
            .unwrap(),
            r#"
Json matcher failed:
  - $: Expected number fitting u16

Actual:
"42""#
        );

        // Test boolean value
        assert_eq!(
            *std::panic::catch_unwind(|| { assert_jm!(Value::Bool(true), get_matcher()) })
                .err()
                .unwrap()
                .downcast::<String>()
                .unwrap(),
            r#"
Json matcher failed:
  - $: Expected number fitting u16

Actual:
true"#
        );

        // Test null value
        assert_eq!(
            *std::panic::catch_unwind(|| { assert_jm!(Value::Null, get_matcher()) })
                .err()
                .unwrap()
                .downcast::<String>()
                .unwrap(),
            r#"
Json matcher failed:
  - $: Expected number fitting u16

Actual:
null"#
        );

        // Test array value
        assert_eq!(
            *std::panic::catch_unwind(|| {
                assert_jm!(Value::Array(vec![Value::Number(42.into())]), get_matcher())
            })
            .err()
            .unwrap()
            .downcast::<String>()
            .unwrap(),
            r#"
Json matcher failed:
  - $: Expected number fitting u16

Actual:
[
  42
]"#
        );

        // Test object value
        assert_eq!(
            *std::panic::catch_unwind(|| {
                assert_jm!(Value::Object(serde_json::Map::new()), get_matcher())
            })
            .err()
            .unwrap()
            .downcast::<String>()
            .unwrap(),
            r#"
Json matcher failed:
  - $: Expected number fitting u16

Actual:
{}"#
        );
    }

    #[test]
    fn test_u16_matcher_floating_point_numbers() {
        let get_matcher = || U16Matcher::new();

        // Test floating point numbers - these should fail because as_i64() returns None for floats
        assert_eq!(
            *std::panic::catch_unwind(|| {
                assert_jm!(
                    Value::Number(serde_json::Number::from_f64(42.5).unwrap()),
                    get_matcher()
                )
            })
            .err()
            .unwrap()
            .downcast::<String>()
            .unwrap(),
            r#"
Json matcher failed:
  - $: Expected number fitting u16

Actual:
42.5"#
        );

        assert_eq!(
            *std::panic::catch_unwind(|| {
                assert_jm!(
                    Value::Number(serde_json::Number::from_f64(0.0).unwrap()),
                    get_matcher()
                )
            })
            .err()
            .unwrap()
            .downcast::<String>()
            .unwrap(),
            r#"
Json matcher failed:
  - $: Expected number fitting u16

Actual:
0.0"#
        );
    }

    #[test]
    fn test_u16_matcher_edge_cases() {
        let get_matcher = || U16Matcher::new();

        // Test boundary values
        assert_jm!(Value::Number(0.into()), get_matcher());
        assert_jm!(Value::Number(65535.into()), get_matcher());

        // Test just outside boundaries
        assert_eq!(
            *std::panic::catch_unwind(|| { assert_jm!(Value::Number((-1).into()), get_matcher()) })
                .err()
                .unwrap()
                .downcast::<String>()
                .unwrap(),
            r#"
Json matcher failed:
  - $: Integer out of bounds for u16

Actual:
-1"#
        );

        assert_eq!(
            *std::panic::catch_unwind(|| {
                assert_jm!(Value::Number(65536.into()), get_matcher())
            })
            .err()
            .unwrap()
            .downcast::<String>()
            .unwrap(),
            r#"
Json matcher failed:
  - $: Integer out of bounds for u16

Actual:
65536"#
        );
    }

    #[test]
    fn test_raw_json_matches_method() {
        let matcher = U16Matcher::new();

        // Test successful match
        assert_eq!(matcher.json_matches(&Value::Number(42.into())), vec![]);

        // Test out of bounds error
        let errors = matcher.json_matches(&Value::Number((-1).into()));
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].to_string(), "$: Integer out of bounds for u16");

        // Test non-numeric error
        let errors = matcher.json_matches(&Value::String("test".to_string()));
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].to_string(), "$: Expected number fitting u16");
    }

    // Tests for allow_strings mode
    #[test]
    fn test_u16_matcher_allow_strings_valid_values() {
        let get_matcher = || U16Matcher::new_allow_strings();
        
        // Test valid u16 string values
        assert_jm!(Value::String("0".to_string()), get_matcher());
        assert_jm!(Value::String("1".to_string()), get_matcher());
        assert_jm!(Value::String("65535".to_string()), get_matcher());
        assert_jm!(Value::String("32768".to_string()), get_matcher());
        assert_jm!(Value::String("100".to_string()), get_matcher());
        assert_jm!(Value::String("42".to_string()), get_matcher());
    }

    #[test]
    fn test_u16_matcher_allow_strings_invalid_string_values() {
        let get_matcher = || U16Matcher::new_allow_strings();
        
        // Test negative string values
        assert_eq!(
            *std::panic::catch_unwind(|| { assert_jm!(Value::String("-1".to_string()), get_matcher()) })
                .err()
                .unwrap()
                .downcast::<String>()
                .unwrap(),
            r#"
Json matcher failed:
  - $: Expected number fitting u16

Actual:
"-1""#
        );

        // Test values above u16 max
        assert_eq!(
            *std::panic::catch_unwind(|| { assert_jm!(Value::String("65536".to_string()), get_matcher()) })
                .err()
                .unwrap()
                .downcast::<String>()
                .unwrap(),
            r#"
Json matcher failed:
  - $: Expected number fitting u16

Actual:
"65536""#
        );

        assert_eq!(
            *std::panic::catch_unwind(|| { assert_jm!(Value::String("100000".to_string()), get_matcher()) })
                .err()
                .unwrap()
                .downcast::<String>()
                .unwrap(),
            r#"
Json matcher failed:
  - $: Expected number fitting u16

Actual:
"100000""#
        );

        // Test non-numeric strings
        assert_eq!(
            *std::panic::catch_unwind(|| { assert_jm!(Value::String("hello".to_string()), get_matcher()) })
                .err()
                .unwrap()
                .downcast::<String>()
                .unwrap(),
            r#"
Json matcher failed:
  - $: Expected number fitting u16

Actual:
"hello""#
        );

        assert_eq!(
            *std::panic::catch_unwind(|| { assert_jm!(Value::String("".to_string()), get_matcher()) })
                .err()
                .unwrap()
                .downcast::<String>()
                .unwrap(),
            r#"
Json matcher failed:
  - $: Expected number fitting u16

Actual:
"""#
        );

        // Test floating point strings
        assert_eq!(
            *std::panic::catch_unwind(|| { assert_jm!(Value::String("42.5".to_string()), get_matcher()) })
                .err()
                .unwrap()
                .downcast::<String>()
                .unwrap(),
            r#"
Json matcher failed:
  - $: Expected number fitting u16

Actual:
"42.5""#
        );

        // Test strings with leading/trailing spaces
        assert_eq!(
            *std::panic::catch_unwind(|| { assert_jm!(Value::String(" 42 ".to_string()), get_matcher()) })
                .err()
                .unwrap()
                .downcast::<String>()
                .unwrap(),
            r#"
Json matcher failed:
  - $: Expected number fitting u16

Actual:
" 42 ""#
        );

        // Test strings with leading zeros (should still work)
        assert_jm!(Value::String("0042".to_string()), get_matcher());
        assert_jm!(Value::String("00000".to_string()), get_matcher());
    }

    #[test]
    fn test_u16_matcher_allow_strings_non_string_values() {
        let get_matcher = || U16Matcher::new_allow_strings();
        
        // Test numeric value (should fail because we expect strings)
        assert_eq!(
            *std::panic::catch_unwind(|| { assert_jm!(Value::Number(42.into()), get_matcher()) })
                .err()
                .unwrap()
                .downcast::<String>()
                .unwrap(),
            r#"
Json matcher failed:
  - $: Expected string fitting u16

Actual:
42"#
        );

        // Test boolean value
        assert_eq!(
            *std::panic::catch_unwind(|| { assert_jm!(Value::Bool(true), get_matcher()) })
                .err()
                .unwrap()
                .downcast::<String>()
                .unwrap(),
            r#"
Json matcher failed:
  - $: Expected string fitting u16

Actual:
true"#
        );

        // Test null value
        assert_eq!(
            *std::panic::catch_unwind(|| { assert_jm!(Value::Null, get_matcher()) })
                .err()
                .unwrap()
                .downcast::<String>()
                .unwrap(),
            r#"
Json matcher failed:
  - $: Expected string fitting u16

Actual:
null"#
        );

        // Test array value
        assert_eq!(
            *std::panic::catch_unwind(|| { assert_jm!(Value::Array(vec![Value::String("42".to_string())]), get_matcher()) })
                .err()
                .unwrap()
                .downcast::<String>()
                .unwrap(),
            r#"
Json matcher failed:
  - $: Expected string fitting u16

Actual:
[
  "42"
]"#
        );

        // Test object value
        assert_eq!(
            *std::panic::catch_unwind(|| { 
                assert_jm!(Value::Object(serde_json::Map::new()), get_matcher()) 
            })
                .err()
                .unwrap()
                .downcast::<String>()
                .unwrap(),
            r#"
Json matcher failed:
  - $: Expected string fitting u16

Actual:
{}"#
        );
    }

    #[test]
    fn test_u16_matcher_allow_strings_edge_cases() {
        let get_matcher = || U16Matcher::new_allow_strings();
        
        // Test boundary values as strings
        assert_jm!(Value::String("0".to_string()), get_matcher());
        assert_jm!(Value::String("65535".to_string()), get_matcher());

        // Test just outside boundaries as strings
        assert_eq!(
            *std::panic::catch_unwind(|| { assert_jm!(Value::String("-1".to_string()), get_matcher()) })
                .err()
                .unwrap()
                .downcast::<String>()
                .unwrap(),
            r#"
Json matcher failed:
  - $: Expected number fitting u16

Actual:
"-1""#
        );

        assert_eq!(
            *std::panic::catch_unwind(|| { assert_jm!(Value::String("65536".to_string()), get_matcher()) })
                .err()
                .unwrap()
                .downcast::<String>()
                .unwrap(),
            r#"
Json matcher failed:
  - $: Expected number fitting u16

Actual:
"65536""#
        );
    }

    #[test]
    fn test_u16_matcher_allow_strings_raw_method() {
        let matcher = U16Matcher::new_allow_strings();
        
        // Test successful string match
        assert_eq!(
            matcher.json_matches(&Value::String("42".to_string())),
            vec![]
        );

        // Test invalid string error
        let errors = matcher.json_matches(&Value::String("invalid".to_string()));
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].to_string(), "$: Expected number fitting u16");

        // Test non-string error
        let errors = matcher.json_matches(&Value::Number(42.into()));
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].to_string(), "$: Expected string fitting u16");
    }

    #[test]
    fn test_u16_matcher_modes_comparison() {
        let number_matcher = U16Matcher::new();
        let string_matcher = U16Matcher::new_allow_strings();
        
        // Valid number should work for number matcher but not string matcher
        assert_eq!(number_matcher.json_matches(&Value::Number(42.into())), vec![]);
        assert_eq!(string_matcher.json_matches(&Value::Number(42.into())).len(), 1);
        
        // Valid string should work for string matcher but not number matcher
        assert_eq!(string_matcher.json_matches(&Value::String("42".to_string())), vec![]);
        assert_eq!(number_matcher.json_matches(&Value::String("42".to_string())).len(), 1);
        
        // Invalid values should fail for both
        assert_eq!(number_matcher.json_matches(&Value::String("invalid".to_string())).len(), 1);
        assert_eq!(string_matcher.json_matches(&Value::String("invalid".to_string())).len(), 1);
        assert_eq!(number_matcher.json_matches(&Value::Number((-1).into())).len(), 1);
        assert_eq!(string_matcher.json_matches(&Value::String("-1".to_string())).len(), 1);
    }
}
