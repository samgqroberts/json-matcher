use crate::{JsonMatcher, JsonMatcherError};

pub struct UuidMatcher;

impl Default for UuidMatcher {
    fn default() -> Self {
        Self::new()
    }
}

impl UuidMatcher {
    pub fn new() -> Self {
        Self
    }
}

impl JsonMatcher for UuidMatcher {
    fn json_matches(&self, value: &serde_json::Value) -> Vec<JsonMatcherError> {
        match value.as_str() {
            Some(s) if s.len() == 36 && s.chars().filter(|&c| c == '-').count() == 4 => vec![],
            Some(_) => vec![JsonMatcherError::at_root("Expected valid UUID format")],
            None => vec![JsonMatcherError::at_root("Expected string for UUID")],
        }
    }
}
