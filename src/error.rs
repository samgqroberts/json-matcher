use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JsonPathElement {
    Root,
    Index(usize),
    Key(String),
}

impl Display for JsonPathElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            JsonPathElement::Root => "$".to_string(),
            JsonPathElement::Index(index) => format!("{}", index),
            JsonPathElement::Key(key) => key.to_string(),
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JsonPath(Vec<JsonPathElement>);

impl Display for JsonPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .map(|e| format!("{}", e))
                .collect::<Vec<String>>()
                .join(".")
        )
    }
}

impl From<Vec<JsonPathElement>> for JsonPath {
    fn from(elements: Vec<JsonPathElement>) -> Self {
        Self(elements)
    }
}

impl JsonPath {
    pub fn extend<T: Into<JsonPath>>(mut self, elements: T) -> Self {
        let mut elements = Into::<JsonPath>::into(elements).0;
        if elements.first() == Some(&JsonPathElement::Root) {
            elements = elements.into_iter().skip(1).collect();
        }
        let path = &mut self.0;
        path.extend(elements);
        self
    }
}

impl Default for JsonPath {
    fn default() -> Self {
        Self(vec![JsonPathElement::Root])
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JsonMatcherError {
    pub path: JsonPath,
    pub message: String,
}

impl JsonMatcherError {
    pub fn at_root<T: Into<String>>(message: T) -> Self {
        let message = message.into();
        Self {
            path: JsonPath::default(),
            message,
        }
    }
}

impl Display for JsonMatcherError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.path, self.message)
    }
}
