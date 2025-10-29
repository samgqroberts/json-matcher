use serde_json::Value;

use crate::{JsonMatcher, JsonMatcherError, JsonPath, JsonPathElement};

pub struct ArrayMatcherRefs<'a> {
    elements: Vec<&'a dyn JsonMatcher>,
}

impl<'a> ArrayMatcherRefs<'a> {
    pub fn new(elements: Vec<&'a dyn JsonMatcher>) -> Self {
        Self { elements }
    }
}

impl JsonMatcher for ArrayMatcherRefs<'_> {
    fn json_matches(&self, value: &Value) -> Vec<JsonMatcherError> {
        let mut errors: Vec<JsonMatcherError> = vec![];
        match value {
            Value::Array(array) => {
                let actual_length = array.len();
                let expected_length = self.elements.len();
                let expected_but_missing_indexes = actual_length..expected_length;
                if !expected_but_missing_indexes.is_empty() {
                    let min = expected_but_missing_indexes
                        .clone()
                        .min()
                        .expect("Expected array length is greater than 0");
                    let max = expected_but_missing_indexes
                        .max()
                        .expect("Expected array length is greater than 0");
                    let error = if min == max {
                        format!("Array is missing index {}", min)
                    } else {
                        format!("Array is missing indexes: {}..{}", min, max)
                    };
                    errors.push(JsonMatcherError::at_root(error));
                }
                let unexpected_indexes = expected_length..actual_length;
                if !unexpected_indexes.is_empty() {
                    let min = unexpected_indexes
                        .clone()
                        .min()
                        .expect("Unexpected array length is greater than 0");
                    let max = unexpected_indexes
                        .max()
                        .expect("Unexpected array length is greater than 0");
                    let error = if min == max {
                        format!("Array has unexpected index {}", min)
                    } else {
                        format!("Array has unexpected indexes: {}..{}", min, max)
                    };
                    errors.push(JsonMatcherError::at_root(error));
                }
                let expected_and_present_indexes = 0..([actual_length, expected_length]
                    .into_iter()
                    .min()
                    .expect("Array is inlined to have length greater than 0"));
                for index in expected_and_present_indexes {
                    let matcher = &self.elements[index];
                    let value = array.get(index).expect("Index in array checked.");
                    let sub_errors = matcher.json_matches(value);
                    for sub_error in sub_errors {
                        let this_path = JsonPath::from(vec![
                            JsonPathElement::Root,
                            JsonPathElement::Index(index),
                        ]);
                        let JsonMatcherError { path, message } = sub_error;
                        let new_path = this_path.extend(path);
                        errors.push(JsonMatcherError {
                            path: new_path,
                            message,
                        });
                    }
                }
            }
            _ => errors.push(JsonMatcherError::at_root("Value is not an array")),
        }
        errors
    }
}

pub struct ArrayMatcher {
    elements: Vec<Box<dyn JsonMatcher>>,
}

impl Default for ArrayMatcher {
    fn default() -> Self {
        Self::new()
    }
}

impl ArrayMatcher {
    pub fn new() -> Self {
        Self { elements: vec![] }
    }

    pub fn of(elements: Vec<Box<dyn JsonMatcher>>) -> Self {
        Self { elements }
    }

    pub fn element(mut self, value: impl JsonMatcher + 'static) -> Self {
        self.elements.push(Box::new(value));
        self
    }
}

impl JsonMatcher for ArrayMatcher {
    fn json_matches(&self, value: &Value) -> Vec<JsonMatcherError> {
        ArrayMatcherRefs::new(
            self.elements
                .iter()
                .map(|x| x.as_ref() as &dyn JsonMatcher)
                .collect(),
        )
        .json_matches(value)
    }
}

impl JsonMatcher for Vec<Box<dyn JsonMatcher>> {
    fn json_matches(&self, value: &Value) -> Vec<JsonMatcherError> {
        ArrayMatcherRefs::new(
            self.iter()
                .map(|x| x.as_ref() as &dyn JsonMatcher)
                .collect(),
        )
        .json_matches(value)
    }
}

impl JsonMatcher for [Box<dyn JsonMatcher>] {
    fn json_matches(&self, value: &Value) -> Vec<JsonMatcherError> {
        ArrayMatcherRefs::new(
            self.iter()
                .map(|x| x.as_ref() as &dyn JsonMatcher)
                .collect(),
        )
        .json_matches(value)
    }
}

impl JsonMatcher for Vec<&dyn JsonMatcher> {
    fn json_matches(&self, value: &Value) -> Vec<JsonMatcherError> {
        ArrayMatcherRefs::new(self.to_vec()).json_matches(value)
    }
}

impl JsonMatcher for [&dyn JsonMatcher] {
    fn json_matches(&self, value: &Value) -> Vec<JsonMatcherError> {
        ArrayMatcherRefs::new(self.to_vec()).json_matches(value)
    }
}

impl<T: JsonMatcher> JsonMatcher for Vec<T> {
    fn json_matches(&self, value: &Value) -> Vec<JsonMatcherError> {
        ArrayMatcherRefs::new(self.iter().map(|x| x as &dyn JsonMatcher).collect())
            .json_matches(value)
    }
}

impl<T: JsonMatcher> JsonMatcher for [T] {
    fn json_matches(&self, value: &Value) -> Vec<JsonMatcherError> {
        ArrayMatcherRefs::new(self.iter().map(|x| x as &dyn JsonMatcher).collect())
            .json_matches(value)
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::test::catch_string_panic;
    use crate::{assert_jm, StringMatcher};

    use super::*;

    #[test]
    fn test_array_matcher() {
        let get_matcher = || {
            ArrayMatcher::new()
                .element(
                    ArrayMatcher::new()
                        .element(StringMatcher::new("one"))
                        .element(StringMatcher::new("two")),
                )
                .element(StringMatcher::new("three"))
        };
        // successful match
        assert_jm!(json!([["one", "two"], "three"]), get_matcher());
        // problem in a matcher under the root array
        assert_eq!(
            catch_string_panic(|| assert_jm!(json!([["one", "two"], "four"]), get_matcher())),
            r#"
Json matcher failed:
  - $.1: Expected string "three" but got "four"

Actual:
[
  [
    "one",
    "two"
  ],
  "four"
]"#
        );
        // problem in a matcher under a nested array
        assert_eq!(
            catch_string_panic(|| assert_jm!(json!([["one", "four"], "three"]), get_matcher())),
            r#"
Json matcher failed:
  - $.0.1: Expected string "two" but got "four"

Actual:
[
  [
    "one",
    "four"
  ],
  "three"
]"#
        );
        // unexpected index in root
        assert_eq!(
            catch_string_panic(|| assert_jm!(
                json!([["one", "two"], "three", "four"]),
                get_matcher()
            )),
            r#"
Json matcher failed:
  - $: Array has unexpected index 2

Actual:
[
  [
    "one",
    "two"
  ],
  "three",
  "four"
]"#
        );
        // unexpected index in nested array
        assert_eq!(
            catch_string_panic(|| assert_jm!(
                json!([["one", "two", "four"], "three"]),
                get_matcher()
            )),
            r#"
Json matcher failed:
  - $.0: Array has unexpected index 2

Actual:
[
  [
    "one",
    "two",
    "four"
  ],
  "three"
]"#
        );
        // multiple issues
        assert_eq!(
            catch_string_panic(|| assert_jm!(json!([[2], "three", "four", "five"]), get_matcher())),
            r#"
Json matcher failed:
  - $: Array has unexpected indexes: 2..3
  - $.0: Array is missing index 1
  - $.0.0: Value is not a string

Actual:
[
  [
    2
  ],
  "three",
  "four",
  "five"
]"#
        );
    }

    #[test]
    fn test_raw_implementations() {
        let matcher: Vec<Box<dyn JsonMatcher>> = vec![Box::new(1), Box::new(2)];
        assert_eq!(matcher.json_matches(&json!([1, 2])), vec![]);
        assert_eq!(
            matcher
                .json_matches(&json!([1, 2, 3]))
                .into_iter()
                .map(|x| x.to_string())
                .collect::<String>(),
            "$: Array has unexpected index 2"
        );
        let matcher: [Box<dyn JsonMatcher>; 2] = [Box::new(1), Box::new(2)];
        assert_eq!(matcher.json_matches(&json!([1, 2])), vec![]);
        assert_eq!(
            matcher
                .json_matches(&json!([1, 2, 3]))
                .into_iter()
                .map(|x| x.to_string())
                .collect::<String>(),
            "$: Array has unexpected index 2"
        );
        let matcher: Vec<&dyn JsonMatcher> = vec![&1, &2];
        assert_eq!(matcher.json_matches(&json!([1, 2])), vec![]);
        assert_eq!(
            matcher
                .json_matches(&json!([1, 2, 3]))
                .into_iter()
                .map(|x| x.to_string())
                .collect::<String>(),
            "$: Array has unexpected index 2"
        );
        let matcher: [&dyn JsonMatcher; 2] = [&1, &2];
        assert_eq!(matcher.json_matches(&json!([1, 2])), vec![]);
        assert_eq!(
            matcher
                .json_matches(&json!([1, 2, 3]))
                .into_iter()
                .map(|x| x.to_string())
                .collect::<String>(),
            "$: Array has unexpected index 2"
        );
        let matcher: Vec<i8> = vec![1, 2];
        assert_eq!(matcher.json_matches(&json!([1, 2])), vec![]);
        assert_eq!(
            matcher
                .json_matches(&json!([1, 2, 3]))
                .into_iter()
                .map(|x| x.to_string())
                .collect::<String>(),
            "$: Array has unexpected index 2"
        );
        let matcher: [i8; 2] = [1, 2];
        assert_eq!(matcher.json_matches(&json!([1, 2])), vec![]);
        assert_eq!(
            matcher
                .json_matches(&json!([1, 2, 3]))
                .into_iter()
                .map(|x| x.to_string())
                .collect::<String>(),
            "$: Array has unexpected index 2"
        );
    }
}
