use indexmap::IndexMap;

use crate::error::ParserError;
use crate::token::*;
use crate::value::*;
pub(crate) struct Parser;

impl Parser {
    pub(crate) fn parse(tokens: Vec<Token>) -> Result<JsonValue, ParserError> {
        let mut tokens = tokens.into_iter().peekable();
        Self::parse_value(&mut tokens)
    }

    fn parse_value(
        tokens: &mut std::iter::Peekable<impl Iterator<Item = Token>>,
    ) -> Result<JsonValue, ParserError> {
        match tokens.peek() {
            Some(Token::LeftBrace) => Self::parse_object(tokens),
            Some(Token::LeftBracket) => Self::parse_array(tokens),
            Some(Token::Str(_)) => Self::parse_string(tokens).map(JsonValue::Str),
            Some(Token::Num(_)) => Self::parse_number(tokens).map(JsonValue::Num),
            Some(Token::True) => tokens
                .next()
                .ok_or(ParserError::UnexpectedEndOfInput)
                .map(|_| JsonValue::Bool(true)),
            Some(Token::False) => tokens
                .next()
                .ok_or(ParserError::UnexpectedEndOfInput)
                .map(|_| JsonValue::Bool(false)),
            Some(Token::Null) => tokens
                .next()
                .ok_or(ParserError::UnexpectedEndOfInput)
                .map(|_| JsonValue::Null),
            Some(_) => Err(ParserError::UnexpectedToken(tokens.next().unwrap())),
            None => Err(ParserError::UnexpectedEndOfInput),
        }
    }

    fn parse_object(
        tokens: &mut std::iter::Peekable<impl Iterator<Item = Token>>,
    ) -> Result<JsonValue, ParserError> {
        let mut object = IndexMap::new();

        // consume the opening '{'
        match tokens.next() {
            Some(Token::LeftBrace) => (),
            Some(token) => return Err(ParserError::UnexpectedToken(token)),
            None => return Err(ParserError::UnexpectedEndOfInput),
        };

        loop {
            match tokens.peek() {
                Some(Token::RightBrace) => {
                    tokens.next(); // consume the closing '}' and break
                    break;
                }
                Some(Token::Str(_)) => {
                    loop {
                        let key = Self::parse_string(tokens)?;
                        match tokens.next() {
                            Some(Token::Colon) => (),
                            Some(token) => return Err(ParserError::ExpectedColon(token)),
                            None => return Err(ParserError::UnexpectedEndOfInput),
                        }
                        let value = Self::parse_value(tokens)?;
                        object.insert(key, value);

                        match tokens.peek() {
                            Some(Token::Comma) => {
                                tokens.next(); // consume the comma and continue parsing the next key-value pair
                            }
                            Some(Token::RightBrace) => {
                                break;
                            }
                            Some(token) => {
                                return Err(ParserError::ExpectedCommaOrClosingBrace(
                                    token.clone(),
                                ));
                            }
                            None => return Err(ParserError::UnexpectedEndOfInput),
                        }
                    }
                }
                Some(token) => return Err(ParserError::UnexpectedToken(token.clone())),
                None => return Err(ParserError::UnexpectedEndOfInput),
            }
        }

        Ok(JsonValue::Object(object))
    }
    fn parse_array(
        tokens: &mut std::iter::Peekable<impl Iterator<Item = Token>>,
    ) -> Result<JsonValue, ParserError> {
        let mut array = Vec::new();

        match tokens.next() {
            Some(Token::LeftBracket) => (),
            Some(token) => return Err(ParserError::UnexpectedToken(token)),
            None => return Err(ParserError::UnexpectedEndOfInput),
        };

        loop {
            match tokens.peek() {
                Some(Token::RightBracket) => {
                    tokens.next(); // consume the closing bracket and break
                    break;
                }
                Some(_) => {
                    let value = Self::parse_value(tokens)?;
                    array.push(value);

                    match tokens.peek() {
                        Some(Token::Comma) => {
                            tokens.next(); // consume the comma and continue parsing the next value
                        }
                        Some(Token::RightBracket) => {
                            tokens.next(); // consume the closing bracket 
                            break;
                        }
                        Some(token) => {
                            return Err(ParserError::ExpectedCommaOrClosingBrace(token.clone()));
                        }
                        None => return Err(ParserError::UnexpectedEndOfInput),
                    }
                }
                None => return Err(ParserError::UnexpectedEndOfInput),
            }
        }
        Ok(JsonValue::Array(array))
    }

    fn parse_string(
        tokens: &mut std::iter::Peekable<impl Iterator<Item = Token>>,
    ) -> Result<String, ParserError> {
        match tokens.next() {
            Some(Token::Str(s)) => Ok(s),
            Some(token) => Err(ParserError::UnexpectedToken(token)),
            None => Err(ParserError::UnexpectedEndOfInput),
        }
    }

    fn parse_number(
        tokens: &mut std::iter::Peekable<impl Iterator<Item = Token>>,
    ) -> Result<f64, ParserError> {
        match tokens.next() {
            Some(Token::Num(n)) => Ok(n),
            Some(token) => Err(ParserError::UnexpectedToken(token)),
            None => Err(ParserError::UnexpectedEndOfInput),
        }
    }
}
