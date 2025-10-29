use serde_json::Value;

use crate::JsonMatcherError;

pub trait JsonMatcher {
    fn json_matches(&self, value: &Value) -> Vec<JsonMatcherError>;
}
