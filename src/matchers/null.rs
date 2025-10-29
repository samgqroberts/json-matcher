use serde_json::Value;

use crate::{JsonMatcher, JsonMatcherError};

pub struct NullMatcher {}

impl Default for NullMatcher {
    fn default() -> Self {
        Self::new()
    }
}

impl NullMatcher {
    pub fn new() -> Self {
        Self {}
    }
}

impl JsonMatcher for NullMatcher {
    fn json_matches(&self, value: &Value) -> Vec<JsonMatcherError> {
        match value {
            Value::Null => vec![],
            _ => vec![JsonMatcherError::at_root("Value is not null")],
        }
    }
}

impl JsonMatcher for () {
    fn json_matches(&self, value: &Value) -> Vec<JsonMatcherError> {
        NullMatcher::new().json_matches(value)
    }
}

#[cfg(test)]
mod tests {
    use crate::assert_jm;

    use super::*;

    #[test]
    fn test_null_matcher() {
        let matcher = NullMatcher::new();
        assert_jm!(Value::Null, matcher);
        assert_eq!(
            *std::panic::catch_unwind(|| {
                assert_jm!(Value::String("world".to_string()), matcher)
            })
            .err()
            .unwrap()
            .downcast::<String>()
            .unwrap(),
            r#"
Json matcher failed:
  - $: Value is not null

Actual:
"world""#
        );
    }

    #[test]
    fn test_raw_implementations() {
        assert_eq!(().json_matches(&Value::Null), vec![]);
        assert_eq!(
            ().json_matches(&Value::Bool(true))
                .into_iter()
                .map(|x| x.to_string())
                .collect::<String>(),
            "$: Value is not null"
        );
    }
}
