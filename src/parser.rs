use crate::error::ParserError;
use crate::token::*;
pub(crate) struct Parser;

impl Parser {
    pub(crate) fn parse(tokens: Vec<Token>) -> Result<Token, ParserError> {
        let mut token = tokens.into_iter().peekable();
        Self::parse_object(&mut token)
    }

    fn parse_value(
        tokens: &mut std::iter::Peekable<impl Iterator<Item = Token>>,
    ) -> Result<Token, ParserError> {
        match tokens.peek() {
            Some(Token::Punc('{')) => Self::parse_object(tokens),
            Some(Token::Punc('[')) => Self::parse_array(tokens),
            Some(Token::Str(_)) => tokens.next().ok_or(ParserError::UnexpectedEndOfInput),
            Some(Token::Num(_)) => tokens.next().ok_or(ParserError::UnexpectedEndOfInput),
            Some(Token::True) => tokens.next().ok_or(ParserError::UnexpectedEndOfInput),
            Some(Token::False) => tokens.next().ok_or(ParserError::UnexpectedEndOfInput),
            Some(Token::Null) => tokens.next().ok_or(ParserError::UnexpectedEndOfInput),
            Some(_) => Err(ParserError::UnexpectedToken(tokens.next().unwrap())),
            None => Err(ParserError::UnexpectedEndOfInput),
        }
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

    fn parse_object(
        tokens: &mut std::iter::Peekable<impl Iterator<Item = Token>>,
    ) -> Result<Token, ParserError> {
        let mut object = std::collections::HashMap::new();

        // consume the opening '{'
        match tokens.next() {
            Some(Token::Punc('{')) => (),
            Some(token) => return Err(ParserError::UnexpectedToken(token)),
            None => return Err(ParserError::UnexpectedEndOfInput),
        };

        loop {
            match tokens.peek() {
                Some(Token::Punc('}')) => {
                    tokens.next(); // consume the closing '}' and break
                    break;
                }
                Some(Token::Str(_)) => {
                    loop {
                        let key = Self::parse_string(tokens)?;
                        match tokens.next() {
                            Some(Token::Punc(':')) => (),
                            Some(token) => return Err(ParserError::ExpectedColon(token)),
                            None => return Err(ParserError::UnexpectedEndOfInput),
                        }
                        let value = Self::parse_value(tokens)?;
                        object.insert(key, value);

                        match tokens.peek() {
                            Some(Token::Punc(',')) => {
                                tokens.next(); // consume the comma and continue parsing the next key-value pair
                            }
                            Some(Token::Punc('}')) => {
                                break;
                            }
                            Some(token) => return Err(ParserError::ExpectedCommaOrClosingBrace(token.clone())),
                            None => return Err(ParserError::UnexpectedEndOfInput),
                        }
                    }
                }
                Some(token) => return Err(ParserError::UnexpectedToken(token.clone())),
                None => return Err(ParserError::UnexpectedEndOfInput),
            }
        };

        Ok(Token::Object(object))
    }

    fn parse_array(
        tokens: &mut std::iter::Peekable<impl Iterator<Item = Token>>,
    ) -> Result<Token, ParserError> {
        let mut array = Vec::new();

        match tokens.next() {
            Some(Token::Punc('[')) => (),
            Some(token) => return Err(ParserError::UnexpectedToken(token)),
            None => return Err(ParserError::UnexpectedEndOfInput),
        };
        
        loop {
            match tokens.peek() {
                Some(Token::Punc(']')) => {
                    tokens.next(); // consume the closing bracket and break
                    break;
                }
                Some(_) => {
                    let value = Self::parse_value(tokens)?;
                    array.push(value);

                    match tokens.peek() {
                        Some(Token::Punc(',')) => {
                            tokens.next(); // consume the comma and continue parsing the next value
                        }
                        Some(Token::Punc(']')) => {
                            tokens.next(); // consume the closing bracket 
                            break;
                        }
                        Some(token) => return Err(ParserError::ExpectedCommaOrClosingBrace(token.clone())),
                        None => return Err(ParserError::UnexpectedEndOfInput),
                    }
                }
                None => return Err(ParserError::UnexpectedEndOfInput),
            }
        } 
        Ok(Token::Array(array))
    }
}
