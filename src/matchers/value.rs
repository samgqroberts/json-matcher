use serde_json::Value;

use crate::{
    ArrayMatcherRefs, BooleanMatcher, IntegerMatcher, JsonMatcher, JsonMatcherError, NullMatcher,
    NumberMatcher, ObjectMatcherRefs, StrMatcher,
};

impl JsonMatcher for Value {
    fn json_matches(&self, value: &Value) -> Vec<JsonMatcherError> {
        match self {
            Value::Null => NullMatcher::new().json_matches(value),
            Value::Bool(x) => BooleanMatcher::exact(*x).json_matches(value),
            Value::Number(number) => match number.as_i64() {
                Some(integer) => IntegerMatcher::new(integer).json_matches(value),
                None => NumberMatcher::new(number.as_f64().unwrap()).json_matches(value),
            },
            Value::String(x) => StrMatcher::new(x).json_matches(value),
            Value::Array(vec) => {
                ArrayMatcherRefs::new(vec.iter().map(|x| x as &dyn JsonMatcher).collect())
                    .json_matches(value)
            }
            Value::Object(map) => ObjectMatcherRefs::new(
                false,
                map.into_iter()
                    .map(|(k, v)| (k.as_str(), v as &dyn JsonMatcher))
                    .collect(),
            )
            .json_matches(value),
        }
    }
}

impl JsonMatcher for &Value {
    fn json_matches(&self, value: &Value) -> Vec<JsonMatcherError> {
        (*self).json_matches(value)
    }
}

#[cfg(test)]
mod tests {
    use serde_json::{json, Number};

    use crate::{ArrayMatcher, ObjectMatcher};

    use super::*;

    #[test]
    fn test_value_json_matcher_impl() {
        // null
        assert_eq!(
            Value::Null.json_matches(&Value::Null),
            NullMatcher::new().json_matches(&Value::Null)
        );
        assert_eq!(
            Value::Null.json_matches(&Value::String("hello".to_owned())),
            NullMatcher::new().json_matches(&Value::String("hello".to_owned()))
        );
        // bool
        assert_eq!(
            Value::Bool(true).json_matches(&Value::Bool(true)),
            BooleanMatcher::exact(true).json_matches(&Value::Bool(true))
        );
        assert_eq!(
            Value::Bool(true).json_matches(&Value::Bool(false)),
            BooleanMatcher::exact(true).json_matches(&Value::Bool(false))
        );
        // number
        assert_eq!(
            Value::Number(Number::from(1)).json_matches(&Value::Number(Number::from(1))),
            IntegerMatcher::new(1).json_matches(&Value::Number(Number::from(1)))
        );
        assert_eq!(
            Value::Number(Number::from(1)).json_matches(&Value::Number(Number::from(2))),
            IntegerMatcher::new(1).json_matches(&Value::Number(Number::from(2)))
        );
        // string
        assert_eq!(
            Value::String("hello".to_string()).json_matches(&Value::String("hello".to_string())),
            StrMatcher::new("hello").json_matches(&Value::String("hello".to_string()))
        );
        assert_eq!(
            Value::String("hello".to_string()).json_matches(&Value::String("world".to_string())),
            StrMatcher::new("hello").json_matches(&Value::String("world".to_string()))
        );
        // array
        assert_eq!(
            json!([1, 2]).json_matches(&json!([1, 2])),
            ArrayMatcher::new()
                .element(1)
                .element(2)
                .json_matches(&json!([1, 2]))
        );
        assert_eq!(
            json!([1, 2]).json_matches(&json!([1, 3])),
            ArrayMatcher::new()
                .element(1)
                .element(2)
                .json_matches(&json!([1, 3]))
        );
        // object
        assert_eq!(
            json!({"a": true}).json_matches(&json!({"a": true})),
            ObjectMatcher::new()
                .field("a", true)
                .json_matches(&json!({"a": true}))
        );
        assert_eq!(
            json!({"a": true}).json_matches(&json!({"a": false})),
            ObjectMatcher::new()
                .field("a", true)
                .json_matches(&json!({"a": false}))
        );
    }
}
