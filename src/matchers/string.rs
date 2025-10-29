use serde_json::Value;

use crate::{JsonMatcher, JsonMatcherError};

pub struct StrMatcher<'a> {
    value: &'a str,
}

impl<'a> StrMatcher<'a> {
    pub fn new(value: &'a str) -> Self {
        Self { value }
    }
}

impl JsonMatcher for StrMatcher<'_> {
    fn json_matches(&self, value: &Value) -> Vec<JsonMatcherError> {
        match value {
            Value::String(actual) => {
                if actual == self.value {
                    vec![]
                } else {
                    vec![JsonMatcherError::at_root(format!(
                        "Expected string \"{}\" but got \"{}\"",
                        self.value, actual
                    ))]
                }
            }
            _ => vec![JsonMatcherError::at_root("Value is not a string")],
        }
    }
}

impl JsonMatcher for &str {
    fn json_matches(&self, value: &Value) -> Vec<JsonMatcherError> {
        StringMatcher::new(*self).json_matches(value)
    }
}

impl JsonMatcher for &String {
    fn json_matches(&self, value: &Value) -> Vec<JsonMatcherError> {
        StrMatcher::new(self.as_str()).json_matches(value)
    }
}

pub struct StringMatcher {
    value: String,
}

impl StringMatcher {
    pub fn new<T: Into<String>>(value: T) -> Self {
        Self {
            value: value.into(),
        }
    }
}

impl JsonMatcher for StringMatcher {
    fn json_matches(&self, value: &Value) -> Vec<JsonMatcherError> {
        StrMatcher::new(&self.value).json_matches(value)
    }
}

impl JsonMatcher for String {
    fn json_matches(&self, value: &Value) -> Vec<JsonMatcherError> {
        StrMatcher::new(self.as_str()).json_matches(value)
    }
}

#[cfg(test)]
mod tests {
    use crate::assert_jm;

    use super::*;

    #[test]
    fn test_string_matcher() {
        let get_matcher = || StringMatcher::new("hello");
        assert_jm!(Value::String("hello".to_string()), get_matcher());
        // not a string
        assert_eq!(
            *std::panic::catch_unwind(|| { assert_jm!(Value::Number(2.into()), get_matcher()) })
                .err()
                .unwrap()
                .downcast::<String>()
                .unwrap(),
            r#"
Json matcher failed:
  - $: Value is not a string

Actual:
2"#
        );
        // is string, but not expected value
        assert_eq!(
            *std::panic::catch_unwind(|| {
                assert_jm!(Value::String("world".to_string()), get_matcher())
            })
            .err()
            .unwrap()
            .downcast::<String>()
            .unwrap(),
            r#"
Json matcher failed:
  - $: Expected string "hello" but got "world"

Actual:
"world""#
        );
    }

    #[test]
    fn test_raw_implementations() {
        assert_eq!(
            "hello".json_matches(&Value::String("hello".to_string())),
            vec![]
        );
        assert_eq!(
            "hello"
                .json_matches(&Value::String("world".to_string()))
                .into_iter()
                .map(|x| x.to_string())
                .collect::<String>(),
            "$: Expected string \"hello\" but got \"world\""
        );
    }
}
