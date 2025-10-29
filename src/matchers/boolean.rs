use serde_json::Value;

use crate::{JsonMatcher, JsonMatcherError};

pub enum BooleanMatcher {
    Exact(bool),
    Any,
}

impl BooleanMatcher {
    pub fn exact(value: bool) -> Self {
        Self::Exact(value)
    }

    pub fn any() -> Self {
        Self::Any
    }
}

impl JsonMatcher for BooleanMatcher {
    fn json_matches(&self, value: &Value) -> Vec<JsonMatcherError> {
        match value {
            Value::Bool(actual) => match self {
                BooleanMatcher::Exact(expected) => {
                    if *actual != *expected {
                        vec![JsonMatcherError::at_root(format!(
                            "Value is not {}",
                            expected
                        ))]
                    } else {
                        vec![]
                    }
                }
                BooleanMatcher::Any => vec![],
            },
            _ => vec![JsonMatcherError::at_root("Value is not a boolean")],
        }
    }
}

impl JsonMatcher for bool {
    fn json_matches(&self, value: &Value) -> Vec<JsonMatcherError> {
        BooleanMatcher::exact(*self).json_matches(value)
    }
}

#[cfg(test)]
mod tests {
    use crate::assert_jm;
    use crate::test::catch_string_panic;

    use super::*;

    #[test]
    fn test_boolean_matcher() {
        let get_matcher = || BooleanMatcher::exact(true);
        assert_jm!(Value::Bool(true), get_matcher());
        // not a boolean
        assert_eq!(
            catch_string_panic(|| assert_jm!(Value::String("bloop".to_string()), get_matcher())),
            r#"
Json matcher failed:
  - $: Value is not a boolean

Actual:
"bloop""#
        );
        // is boolean, but not expected value
        assert_eq!(
            catch_string_panic(|| assert_jm!(Value::Bool(false), get_matcher())),
            r#"
Json matcher failed:
  - $: Value is not true

Actual:
false"#
        );
    }

    #[test]
    fn test_raw_implementations() {
        assert_eq!(true.json_matches(&Value::Bool(true)), vec![]);
        assert_eq!(
            true.json_matches(&Value::Bool(false))
                .into_iter()
                .map(|x| x.to_string())
                .collect::<String>(),
            "$: Value is not true"
        );
    }
}
