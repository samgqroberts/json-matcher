//! Declarative JSON value matching for testing.
//!
//! This library provides a flexible way to assert that JSON values match expected patterns,
//! supporting both exact matching and flexible matchers for common types like UUIDs, dates,
//! and arbitrary values.
//!
//! # Basic Usage
//!
//! Use the [`assert_jm!`] macro to match JSON values against expected patterns:
//!
//! ```
//! use serde_json::json;
//! use json_matcher::assert_jm;
//!
//! let response = json!({
//!     "status": "success",
//!     "count": 42
//! });
//!
//! // Exact match using inline JSON syntax
//! assert_jm!(response, {
//!     "status": "success",
//!     "count": 42
//! });
//! ```
//!
//! # Using Matchers
//!
//! For flexible matching, use matcher types like [`AnyMatcher`], [`UuidMatcher`], or [`U16Matcher`]:
//!
//! ```
//! use serde_json::json;
//! use json_matcher::{assert_jm, AnyMatcher, UuidMatcher};
//!
//! let response = json!({
//!     "id": "550e8400-e29b-41d4-a716-446655440000",
//!     "timestamp": "2024-01-15T10:30:00Z",
//!     "value": 123
//! });
//!
//! assert_jm!(response, {
//!     "id": UuidMatcher::new(),
//!     "timestamp": AnyMatcher::not_null(),
//!     "value": 123
//! });
//! ```
//!
//! # Error Reporting
//!
//! When assertions fail, [`assert_jm!`] reports all errors found (not just the first) and displays
//! the actual JSON value for debugging:
//!
//! ```should_panic
//! use serde_json::json;
//! use json_matcher::{assert_jm, UuidMatcher};
//!
//! let response = json!({
//!     "id": "not-a-uuid",
//!     "name": "Alice",
//!     "age": 25
//! });
//!
//! // This will panic with a detailed error message showing:
//! // - All validation errors ($.id and $.name mismatches)
//! // - The full actual JSON value
//! assert_jm!(response, {
//!     "id": UuidMatcher::new(),
//!     "name": "Bob",
//!     "age": 25
//! });
//! // Output:
//! // Json matcher failed:
//! //   - $.id: Expected valid UUID format
//! //   - $.name: Expected string "Bob" but got "Alice"
//! //
//! // Actual:
//! // {
//! //   "id": "not-a-uuid",
//! //   "name": "Alice",
//! //   "age": 25
//! // }
//! ```
//!
//! # Custom Matchers
//!
//! Create custom matchers by implementing the [`JsonMatcher`] trait:
//!
//! ```
//! use serde_json::{json, Value};
//! use json_matcher::{assert_jm, JsonMatcher, JsonMatcherError};
//!
//! struct OnlyVowels;
//!
//! impl JsonMatcher for OnlyVowels {
//!     fn json_matches(&self, value: &Value) -> Vec<JsonMatcherError> {
//!         match value.as_str() {
//!             Some(s) if s.chars().all(|c| "aeiouAEIOU".contains(c)) => vec![],
//!             Some(_) => vec![JsonMatcherError::at_root("String contains non-vowel characters")],
//!             None => vec![JsonMatcherError::at_root("Expected string")],
//!         }
//!     }
//! }
//!
//! let data = json!({
//!     "sound": "aeiou",
//!     "count": 5
//! });
//!
//! assert_jm!(data, {
//!     "sound": OnlyVowels,
//!     "count": 5
//! });
//! ```
//!

mod matchers;
pub use matchers::*;
mod error;
pub use error::*;
mod json_matcher;
pub use json_matcher::*;
mod macros;
mod uuid_matcher;
pub use uuid_matcher::*;
mod u16_matcher;
pub use u16_matcher::*;

#[cfg(feature = "datetime")]
pub mod datetime;

#[cfg(test)]
pub mod test;
