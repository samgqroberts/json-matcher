use std::collections::{HashMap, HashSet};

use serde_json::Value;

use crate::{JsonMatcher, JsonMatcherError, JsonPath, JsonPathElement};

pub struct ObjectMatcherRefs<'a> {
    allow_unexpected_keys: bool,
    fields: HashMap<&'a str, &'a dyn JsonMatcher>,
}

impl<'a> ObjectMatcherRefs<'a> {
    pub fn new(allow_unexpected_keys: bool, fields: HashMap<&'a str, &'a dyn JsonMatcher>) -> Self {
        Self {
            allow_unexpected_keys,
            fields,
        }
    }
}

impl JsonMatcher for ObjectMatcherRefs<'_> {
    fn json_matches(&self, value: &Value) -> Vec<JsonMatcherError> {
        let mut errors: Vec<JsonMatcherError> = vec![];
        match value {
            Value::Object(map) => {
                let actual_keys = map.keys().map(|x| x.as_str()).collect::<HashSet<&str>>();
                let expected_keys = self.fields.keys().copied().collect::<HashSet<&str>>();
                let mut expected_but_missing = expected_keys
                    .difference(&actual_keys)
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>();
                if !expected_but_missing.is_empty() {
                    expected_but_missing.sort();
                    errors.push(JsonMatcherError::at_root(format!(
                        "Object is missing keys: {}",
                        expected_but_missing.join(", ")
                    )));
                }
                if !self.allow_unexpected_keys {
                    let mut unexpected = actual_keys
                        .difference(&expected_keys)
                        .map(|x| x.to_string())
                        .collect::<Vec<_>>();
                    if !unexpected.is_empty() {
                        unexpected.sort();
                        errors.push(JsonMatcherError::at_root(format!(
                            "Object has unexpected keys: {}",
                            unexpected.join(", ")
                        )));
                    }
                }
                let mut expected_and_present = expected_keys
                    .intersection(&actual_keys).copied()
                    .collect::<Vec<&str>>();
                expected_and_present.sort();
                for key in expected_and_present {
                    let matcher = self.fields.get(key).expect("Key in fields checked.");
                    let value = map.get(key).expect("Key in map checked.");
                    for sub_error in matcher.json_matches(value) {
                        let this_path = JsonPath::from(vec![
                            JsonPathElement::Root,
                            JsonPathElement::Key(key.to_owned()),
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
            _ => errors.push(JsonMatcherError::at_root("Value is not an object")),
        }
        errors
    }
}

pub struct ObjectMatcher {
    allow_unexpected_keys: bool,
    fields: HashMap<String, Box<dyn JsonMatcher>>,
}

impl Default for ObjectMatcher {
    fn default() -> Self {
        Self::new()
    }
}

impl ObjectMatcher {
    pub fn new() -> Self {
        Self {
            allow_unexpected_keys: false,
            fields: HashMap::new(),
        }
    }

    pub fn of(fields: HashMap<String, Box<dyn JsonMatcher>>) -> Self {
        Self {
            allow_unexpected_keys: false,
            fields,
        }
    }

    pub fn allow_unexpected_keys(mut self) -> Self {
        self.allow_unexpected_keys = true;
        self
    }

    pub fn field(mut self, key: &str, value: impl JsonMatcher + 'static) -> Self {
        self.fields.insert(key.to_string(), Box::new(value));
        self
    }
}

impl JsonMatcher for ObjectMatcher {
    fn json_matches(&self, value: &Value) -> Vec<JsonMatcherError> {
        ObjectMatcherRefs::new(
            self.allow_unexpected_keys,
            self.fields
                .iter()
                .map(|(k, v)| (k.as_str(), v.as_ref() as &dyn JsonMatcher))
                .collect(),
        )
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
    fn test_object_matcher() {
        let get_matcher = || {
            ObjectMatcher::new()
                .field(
                    "a",
                    ObjectMatcher::new()
                        .field("aa", StringMatcher::new("one"))
                        .field("ab", StringMatcher::new("two")),
                )
                .field("b", StringMatcher::new("three"))
        };
        // successful match
        assert_jm!(
            json!({
                "a": {
                    "aa": "one",
                    "ab": "two"
                },
                "b": "three"
            }),
            get_matcher()
        );
        // problem in a matcher under the root object
        assert_eq!(
            catch_string_panic(|| assert_jm!(
                json!({
                    "a": {
                        "aa": "one",
                        "ab": "two"
                    },
                    "b": "four"
                }),
                get_matcher()
            )),
            r#"
Json matcher failed:
  - $.b: Expected string "three" but got "four"

Actual:
{
  "a": {
    "aa": "one",
    "ab": "two"
  },
  "b": "four"
}"#
        );
        // problem in a matcher under a nested object
        assert_eq!(
            catch_string_panic(|| assert_jm!(
                json!({
                    "a": {
                        "aa": "one",
                        "ab": "four"
                    },
                    "b": "three"
                }),
                get_matcher()
            )),
            r#"
Json matcher failed:
  - $.a.ab: Expected string "two" but got "four"

Actual:
{
  "a": {
    "aa": "one",
    "ab": "four"
  },
  "b": "three"
}"#
        );
        // unexpected key in root object
        assert_eq!(
            catch_string_panic(|| assert_jm!(
                json!({
                "a": {
                    "aa": "one",
                    "ab": "two"
                },
                "b": "three",
                "c": "four"
                }),
                get_matcher()
            )),
            r#"
Json matcher failed:
  - $: Object has unexpected keys: c

Actual:
{
  "a": {
    "aa": "one",
    "ab": "two"
  },
  "b": "three",
  "c": "four"
}"#
        );
        // unexpected key in nested object
        assert_eq!(
            catch_string_panic(|| assert_jm!(
                json!({
                "a": {
                    "aa": "one",
                    "ab": "two",
                    "c": "four"
                },
                "b": "three",
                }),
                get_matcher()
            )),
            r#"
Json matcher failed:
  - $.a: Object has unexpected keys: c

Actual:
{
  "a": {
    "aa": "one",
    "ab": "two",
    "c": "four"
  },
  "b": "three"
}"#
        );
        // multiple issues
        assert_eq!(
            catch_string_panic(|| assert_jm!(
                json!({
                "a": {
                    "aa": 2,
                    "c": "four",
                },
                "d": "five",
                "e": "six"
                }),
                get_matcher()
            )),
            r#"
Json matcher failed:
  - $: Object is missing keys: b
  - $: Object has unexpected keys: d, e
  - $.a: Object is missing keys: ab
  - $.a: Object has unexpected keys: c
  - $.a.aa: Value is not a string

Actual:
{
  "a": {
    "aa": 2,
    "c": "four"
  },
  "d": "five",
  "e": "six"
}"#
        );
    }

    #[test]
    fn test_object_matcher_permissive() {
        assert_jm!(
            json!({
                "a": 1,
                "b": 2
            }),
            ObjectMatcher::new().allow_unexpected_keys().field("a", 1)
        );
        // still fails if expected key is missing
        assert_eq!(
            catch_string_panic(|| assert_jm!(
                json!({
                "b": 2
                }),
                ObjectMatcher::new().allow_unexpected_keys().field("a", 1)
            )),
            r#"
Json matcher failed:
  - $: Object is missing keys: a

Actual:
{
  "b": 2
}"#
        );
    }
}
