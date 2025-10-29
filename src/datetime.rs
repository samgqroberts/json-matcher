use crate::{JsonMatcher, JsonMatcherError};
use chrono::{DateTime, Duration, FixedOffset, Utc};
use chrono_tz::Tz;
use serde_json::Value;

fn parse_datetime_from_string(
    s: &str,
    timezone: Option<&str>,
) -> Result<DateTime<FixedOffset>, String> {
    let datetime = match DateTime::parse_from_rfc3339(s) {
        Ok(x) => x,
        Err(e) => {
            // if original parse fails, this might be because value does not have its own timezone, which parse_from_rfc3339 expects
            // try to add UTC timezone first.
            let parsed = DateTime::parse_from_rfc3339(&(s.to_owned() + "Z")).map_err(|_| {
                // if this fails, then the value is not a valid RFC 3339 timestamp
                // return original error
                format!("Value cannot be parsed as an RFC 3339 timestamp: {e}")
            })?;
            // this succeeded, now if type has a timezone, we need to calculate the offset
            let corrected = match timezone.as_ref() {
                None => parsed,
                Some(tz) => match tz.parse::<Tz>() {
                    Ok(tz) => {
                        // the timezone string in the type is a valid timezone name
                        // so we interpret the original parsed value as if it were already in this timezone
                        let with_timezone: DateTime<Tz> =
                            parsed.naive_utc().and_local_timezone(tz).unwrap();
                        with_timezone.fixed_offset()
                    }
                    Err(_) => {
                        // the timezone string in the type is not a valid timezone name
                        // so just return the utc-parsed value
                        parsed
                    }
                },
            };
            corrected
        }
    };
    Ok(datetime)
}

pub struct DateTimeStringMatcher {
    lower_bound: Option<DateTime<Utc>>,
    lower_bound_inclusive: bool,
    upper_bound: Option<DateTime<Utc>>,
    upper_bound_inclusive: bool,
}

impl DateTimeStringMatcher {
    pub fn recent_utc() -> Self {
        Self {
            lower_bound: Some(Utc::now() - Duration::minutes(1)),
            lower_bound_inclusive: true,
            upper_bound: Some(Utc::now()),
            upper_bound_inclusive: true,
        }
    }
}

impl JsonMatcher for DateTimeStringMatcher {
    fn json_matches(&self, value: &Value) -> Vec<JsonMatcherError> {
        let Value::String(as_str) = value else {
            return vec![JsonMatcherError::at_root(
                "Datetime value needs to be a string",
            )];
        };
        let datetime = match parse_datetime_from_string(as_str, None) {
            Ok(parsed) => parsed,
            Err(err) => {
                return vec![JsonMatcherError::at_root(format!(
                    "Could not parse string as rfc3339 datetime: {}",
                    err
                ))];
            }
        };
        if datetime.offset().utc_minus_local() != 0 {
            return vec![JsonMatcherError::at_root("Datetime is not in UTC")];
        }
        if let Some(upper_bound) = self.upper_bound {
            if self.upper_bound_inclusive {
                if datetime.timestamp() > upper_bound.timestamp() {
                    return vec![JsonMatcherError::at_root("Datetime is after upper bound")];
                }
            } else if datetime.timestamp() >= upper_bound.timestamp() {
                return vec![JsonMatcherError::at_root(
                    "Datetime is after or equal to upper bound",
                )];
            }
        }
        if let Some(lower_bound) = self.lower_bound {
            if self.lower_bound_inclusive {
                if datetime.timestamp() < lower_bound.timestamp() {
                    return vec![JsonMatcherError::at_root(format!(
                        "Datetime is before lower bound of {}",
                        lower_bound.to_rfc3339()
                    ))];
                }
            } else if datetime.timestamp() <= lower_bound.timestamp() {
                return vec![JsonMatcherError::at_root(
                    "Datetime is before or equal to lower bound",
                )];
            }
        }
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use crate::assert_jm;
    use serde_json::json;

    use super::*;

    #[test]
    fn test_date_time_string_matcher() {
        let lower_bound = DateTime::parse_from_rfc3339("2024-01-05T10:00:00Z")
            .unwrap()
            .naive_utc()
            .and_utc();
        let upper_bound = DateTime::parse_from_rfc3339("2024-01-05T11:00:00Z")
            .unwrap()
            .naive_utc()
            .and_utc();
        let matcher = DateTimeStringMatcher {
            lower_bound: Some(lower_bound),
            lower_bound_inclusive: true,
            upper_bound: Some(upper_bound),
            upper_bound_inclusive: true,
        };
        // success cases
        assert_jm!(json!("2024-01-05T10:00:00Z"), matcher);
        assert_jm!(json!("2024-01-05T10:30:00Z"), matcher);
        assert_jm!(json!("2024-01-05T11:00:00Z"), matcher);
        // failure cases
        assert_eq!(
            matcher.json_matches(&json!(2)),
            vec![JsonMatcherError::at_root(
                "Datetime value needs to be a string"
            )]
        );
        assert_eq!(
            matcher.json_matches(&json!("bloop")),
            vec![JsonMatcherError::at_root(
                "Could not parse string as rfc3339 datetime: Value cannot be parsed as an RFC 3339 timestamp: input contains invalid characters"
            )]
        );
        assert_eq!(
            matcher.json_matches(&json!("2024-22-05T10:00:00Z")),
            vec![JsonMatcherError::at_root(
                "Could not parse string as rfc3339 datetime: Value cannot be parsed as an RFC 3339 timestamp: input is out of range"
            )]
        );
        assert_eq!(
            matcher.json_matches(&json!("2024-01-05T09:59:59Z")),
            vec![JsonMatcherError::at_root(
                "Datetime is before lower bound of 2024-01-05T10:00:00+00:00"
            )]
        );
        assert_eq!(
            matcher.json_matches(&json!("2024-01-05T11:00:01Z")),
            vec![JsonMatcherError::at_root("Datetime is after upper bound")]
        );
        assert_eq!(
            matcher.json_matches(&json!("2024-01-05T11:00:01-08:00")),
            vec![JsonMatcherError::at_root("Datetime is not in UTC")]
        );
    }
}
