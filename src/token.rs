use std::collections::HashMap;

#[derive(Debug)]
pub(crate) enum TokenKind {
    Object(HashMap<String, Token>),
    Array(Vec<Token>),
    Punc(char),
    Num(f64),
    Str(String),
    True,
    False,
    Null,
}

#[derive(Debug)]
pub(crate) struct Token {
    pub kind: TokenKind,
}
