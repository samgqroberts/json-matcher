use serde_json::Value;

use crate::{JsonMatcher, JsonMatcherError};

pub struct AnyMatcher {
    not_null: bool,
}

impl Default for AnyMatcher {
    fn default() -> Self {
        Self::new()
    }
}

impl AnyMatcher {
    pub fn new() -> Self {
        Self { not_null: false }
    }

    pub fn not_null() -> Self {
        Self { not_null: true }
    }
}

impl JsonMatcher for AnyMatcher {
    fn json_matches(&self, value: &Value) -> Vec<JsonMatcherError> {
        if self.not_null && value.is_null() {
            vec![JsonMatcherError::at_root("Expected non-null value")]
        } else {
            vec![]
        }
    }
}
