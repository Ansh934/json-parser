use crate::token::Token;
use thiserror::Error;

#[derive(Debug, Error)]
pub(crate) enum LexerError {
    #[error("Unexpected character: {0}")]
    UnexpectedCharacter(char),
    #[error("Unterminated string")]
    UnterminatedString,
    #[error("Invalid number: {0}")]
    InvalidNumber(String),
    #[error("Unexpected keyword: {0}")]
    UnexpectedKeyword(String),
    #[error("Unexpected end of input")]
    UnexpectedEndOfInput,
    #[error("Invalid escape sequence: \\{0}")]
    InvalidEscapeSequence(char),
    #[error("Invalid Unicode escape sequence: \\u{0}")]
    InvalidUnicodeEscapeSequence(String),
}

#[derive(Debug, Error)]
pub(crate) enum ParserError {
    #[error("Unexpected token: {0:?}")]
    UnexpectedToken(Token),
    #[error("Unexpected end of input")]
    UnexpectedEndOfInput,
    #[error("Expected ':' after key in object, got: {0:?}")]
    ExpectedColon(Token),
    #[error("Expected ',' or '}}' after key-value pair in object, got: {0:?}")]
    ExpectedCommaOrClosingBrace(Token),
}
