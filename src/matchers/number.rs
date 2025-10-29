use serde_json::Value;

use crate::{JsonMatcher, JsonMatcherError};

pub struct IntegerMatcher {
    value: i64,
}

impl IntegerMatcher {
    pub fn new(value: i64) -> Self {
        Self { value }
    }
}

impl JsonMatcher for IntegerMatcher {
    fn json_matches(&self, value: &Value) -> Vec<JsonMatcherError> {
        match value {
            Value::Number(num) => {
                let Some(actual) = num.as_i64() else {
                    return vec![JsonMatcherError::at_root(format!(
                        "Expected integer {} but got float {}",
                        self.value, num
                    ))];
                };
                if actual == self.value {
                    vec![]
                } else {
                    vec![JsonMatcherError::at_root(format!(
                        "Expected integer {} but got {}",
                        self.value, actual
                    ))]
                }
            }
            _ => vec![JsonMatcherError::at_root("Value is not an integer")],
        }
    }
}

impl JsonMatcher for i8 {
    fn json_matches(&self, value: &Value) -> Vec<JsonMatcherError> {
        IntegerMatcher::new(*self as i64).json_matches(value)
    }
}

impl JsonMatcher for i16 {
    fn json_matches(&self, value: &Value) -> Vec<JsonMatcherError> {
        IntegerMatcher::new(*self as i64).json_matches(value)
    }
}

impl JsonMatcher for i32 {
    fn json_matches(&self, value: &Value) -> Vec<JsonMatcherError> {
        IntegerMatcher::new(*self as i64).json_matches(value)
    }
}

impl JsonMatcher for i64 {
    fn json_matches(&self, value: &Value) -> Vec<JsonMatcherError> {
        IntegerMatcher::new(*self).json_matches(value)
    }
}

impl JsonMatcher for u8 {
    fn json_matches(&self, value: &Value) -> Vec<JsonMatcherError> {
        IntegerMatcher::new(*self as i64).json_matches(value)
    }
}

impl JsonMatcher for u16 {
    fn json_matches(&self, value: &Value) -> Vec<JsonMatcherError> {
        IntegerMatcher::new(*self as i64).json_matches(value)
    }
}

impl JsonMatcher for u32 {
    fn json_matches(&self, value: &Value) -> Vec<JsonMatcherError> {
        IntegerMatcher::new(*self as i64).json_matches(value)
    }
}

pub struct NumberMatcher {
    number: f64,
}

impl NumberMatcher {
    pub fn new(value: f64) -> Self {
        Self { number: value }
    }
}

impl JsonMatcher for NumberMatcher {
    fn json_matches(&self, value: &Value) -> Vec<JsonMatcherError> {
        match value {
            Value::Number(num) => {
                let Some(actual) = num.as_f64() else {
                    return vec![JsonMatcherError::at_root(format!(
                        "Expected float {} but got integer {}",
                        self.number, num
                    ))];
                };
                if actual == self.number {
                    vec![]
                } else {
                    vec![JsonMatcherError::at_root(format!(
                        "Expected float {} but got {}",
                        self.number, actual
                    ))]
                }
            }
            _ => vec![JsonMatcherError::at_root("Value is not a float")],
        }
    }
}

impl JsonMatcher for f32 {
    fn json_matches(&self, value: &Value) -> Vec<JsonMatcherError> {
        NumberMatcher::new(*self as f64).json_matches(value)
    }
}

impl JsonMatcher for f64 {
    fn json_matches(&self, value: &Value) -> Vec<JsonMatcherError> {
        NumberMatcher::new(*self).json_matches(value)
    }
}

#[cfg(test)]
mod tests {
    use serde_json::Number;

    use crate::assert_jm;

    use crate::test::catch_string_panic;

    use super::*;

    #[test]
    fn test_integer_matcher() {
        let get_matcher = || IntegerMatcher::new(4);
        assert_jm!(Value::Number(4.into()), get_matcher());
        // not a number
        assert_eq!(
            catch_string_panic(|| assert_jm!(Value::String("bloop".to_string()), get_matcher())),
            r#"
Json matcher failed:
  - $: Value is not an integer

Actual:
"bloop""#
        );
        // is number, but not an integer
        assert_eq!(
            catch_string_panic(|| assert_jm!(
                Value::Number(Number::from_f64(2.2).unwrap()),
                get_matcher()
            )),
            r#"
Json matcher failed:
  - $: Expected integer 4 but got float 2.2

Actual:
2.2"#
        );
        // is integer, but not expected value
        assert_eq!(
            catch_string_panic(|| assert_jm!(Value::Number(2.into()), get_matcher())),
            r#"
Json matcher failed:
  - $: Expected integer 4 but got 2

Actual:
2"#
        );
    }

    #[test]
    fn test_number_matcher() {
        let get_matcher = || NumberMatcher::new(4.0);
        // is expected value as float
        assert_jm!(Value::Number(Number::from_f64(4.0).unwrap()), get_matcher());
        // is expected value as integer
        assert_jm!(Value::Number(4i64.into()), get_matcher());
        // not a number
        assert_eq!(
            catch_string_panic(|| assert_jm!(Value::String("bloop".to_string()), get_matcher())),
            r#"
Json matcher failed:
  - $: Value is not a float

Actual:
"bloop""#
        );
        // is float, but not expected value
        assert_eq!(
            catch_string_panic(|| assert_jm!(
                Value::Number(Number::from_f64(7.2).unwrap()),
                get_matcher()
            )),
            r#"
Json matcher failed:
  - $: Expected float 4 but got 7.2

Actual:
7.2"#
        );
    }

    #[test]
    fn test_raw_implementations() {
        // i8
        assert_eq!(4i8.json_matches(&Value::Number(4.into())), vec![]);
        assert_eq!(
            4i8.json_matches(&Value::Number(5.into()))
                .into_iter()
                .map(|e| e.to_string())
                .collect::<String>(),
            "$: Expected integer 4 but got 5"
        );
        // i16
        assert_eq!(4i16.json_matches(&Value::Number(4.into())), vec![]);
        assert_eq!(
            4i16.json_matches(&Value::Number(5.into()))
                .into_iter()
                .map(|e| e.to_string())
                .collect::<String>(),
            "$: Expected integer 4 but got 5"
        );
        // i32
        assert_eq!(4i32.json_matches(&Value::Number(4.into())), vec![]);
        assert_eq!(
            4i32.json_matches(&Value::Number(5.into()))
                .into_iter()
                .map(|e| e.to_string())
                .collect::<String>(),
            "$: Expected integer 4 but got 5"
        );
        // i64
        assert_eq!(4i64.json_matches(&Value::Number(4.into())), vec![]);
        assert_eq!(
            4i64.json_matches(&Value::Number(5.into()))
                .into_iter()
                .map(|e| e.to_string())
                .collect::<String>(),
            "$: Expected integer 4 but got 5"
        );
        // u8
        assert_eq!(4u8.json_matches(&Value::Number(4.into())), vec![]);
        assert_eq!(
            4u8.json_matches(&Value::Number(5.into()))
                .into_iter()
                .map(|e| e.to_string())
                .collect::<String>(),
            "$: Expected integer 4 but got 5"
        );
        // u16
        assert_eq!(4u16.json_matches(&Value::Number(4.into())), vec![]);
        assert_eq!(
            4u16.json_matches(&Value::Number(5.into()))
                .into_iter()
                .map(|e| e.to_string())
                .collect::<String>(),
            "$: Expected integer 4 but got 5"
        );
        // u32
        assert_eq!(4u32.json_matches(&Value::Number(4.into())), vec![]);
        assert_eq!(
            4u32.json_matches(&Value::Number(5.into()))
                .into_iter()
                .map(|e| e.to_string())
                .collect::<String>(),
            "$: Expected integer 4 but got 5"
        );
        // f32
        assert_eq!(4f32.json_matches(&Value::Number(4.into())), vec![]);
        assert_eq!(
            4f32.json_matches(&Value::Number(5.into()))
                .into_iter()
                .map(|e| e.to_string())
                .collect::<String>(),
            "$: Expected float 4 but got 5"
        );
        // f64
        assert_eq!(4f64.json_matches(&Value::Number(4.into())), vec![]);
        assert_eq!(
            4f64.json_matches(&Value::Number(5.into()))
                .into_iter()
                .map(|e| e.to_string())
                .collect::<String>(),
            "$: Expected float 4 but got 5"
        );
    }
}
