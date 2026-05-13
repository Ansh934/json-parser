use indexmap::IndexMap;

#[derive(Debug, Clone)]
pub(crate) enum Token {
    Object(IndexMap<String, Token>),
    Array(Vec<Token>),
    Punc(char),
    Num(f64),
    Str(String),
    True,
    False,
    Null,
}
