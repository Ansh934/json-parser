use std::collections::HashMap;

#[derive(Debug)]
pub(crate) enum Token{
    Object(HashMap<String, Token>),
    Array(Vec<Token>),
    Punc(char),
    Num(f64),
    Str(String),
    True,
    False,
    Null,
}
