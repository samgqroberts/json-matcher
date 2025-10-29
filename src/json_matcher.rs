use serde_json::Value;

use crate::JsonMatcherError;

/// Core trait for implementing JSON value matchers.
///
/// Implement this trait to create custom validation logic for JSON values.
/// The trait requires a single method [`json_matches`](JsonMatcher::json_matches)
/// that returns a vector of errors - an empty vector indicates a successful match.
///
/// # Example
///
/// ```
/// use serde_json::{json, Value};
/// use json_matcher::{assert_jm, JsonMatcher, JsonMatcherError};
///
/// struct OnlyVowels;
///
/// impl JsonMatcher for OnlyVowels {
///     fn json_matches(&self, value: &Value) -> Vec<JsonMatcherError> {
///         match value.as_str() {
///             Some(s) if s.chars().all(|c| "aeiouAEIOU".contains(c)) => vec![],
///             Some(_) => vec![JsonMatcherError::at_root("String contains non-vowel characters")],
///             None => vec![JsonMatcherError::at_root("Expected string")],
///         }
///     }
/// }
///
/// let data = json!({
///     "sound": "aeiou",
///     "count": 5
/// });
///
/// assert_jm!(data, {
///     "sound": OnlyVowels,
///     "count": 5
/// });
/// ```
pub trait JsonMatcher {
    fn json_matches(&self, value: &Value) -> Vec<JsonMatcherError>;
}
