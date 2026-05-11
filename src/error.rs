use thiserror::Error;
use crate::token::Token;

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
}

#[derive(Debug, Error)]
pub(crate) enum ParserError {
    #[error("Unexpected token: {0:?}")]
    UnexpectedToken(Token),
    #[error("Unexpected end of input")]
    UnexpectedEndOfInput,
    #[error("Expected string key in object, got: {0:?}")]
    ExpectedStringKey(Token),
    #[error("Expected ':' after key in object, got: {0:?}")]
    ExpectedColon(Token),
    #[error("Expected ',' or '}}' after key-value pair in object, got: {0:?}")]
    ExpectedCommaOrClosingBrace(Token),
    #[error("Expected ']' after array, got: {0:?}")]
    ExpectedClosingBracket(Token),
    #[error("Expected value in array, got: {0:?}")]
    ExpectedValueInArray(Token),
}