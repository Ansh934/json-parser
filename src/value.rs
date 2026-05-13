// Create a new file: src/value.rs (or put it in parser.rs)
use indexmap::IndexMap;

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum JsonValue {
    Object(IndexMap<String, JsonValue>),
    Array(Vec<JsonValue>),
    Str(String),
    Num(f64),
    Bool(bool),
    Null,
}