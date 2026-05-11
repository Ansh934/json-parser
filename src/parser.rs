use crate::token::{self, *};
pub(crate) struct Parser;

impl Parser {
    pub(crate) fn parse(tokens: Vec<Token>) -> Token {
        let mut token = tokens.into_iter().peekable();
        Self::parse_object(&mut token)
    }

    fn parse_value(tokens: &mut std::iter::Peekable<impl Iterator<Item = Token>>) -> Token {
        match tokens.peek() {
            Some(Token {
                kind: TokenKind::Punc('{'),
            }) => Self::parse_object(tokens),
            Some(Token {
                kind: TokenKind::Punc('['),
            }) => Self::parse_array(tokens),
            Some(Token {
                kind: TokenKind::Str(_),
            }) => tokens.next().unwrap(),
            Some(Token {
                kind: TokenKind::Num(_),
            }) => tokens.next().unwrap(),
            Some(Token {
                kind: TokenKind::True,
            }) => tokens.next().unwrap(),
            Some(Token {
                kind: TokenKind::False,
            }) => tokens.next().unwrap(),
            Some(Token {
                kind: TokenKind::Null,
            }) => tokens.next().unwrap(),
            _ => panic!("Unexpected token: {:?}", tokens.peek()),
        }
    }

    fn parse_string(
        tokens: &mut std::iter::Peekable<impl Iterator<Item = Token>>,
    ) -> Option<String> {
        match tokens.peek() {
            Some(Token {
                kind: TokenKind::Str(_),
            }) => {
                if let Some(Token { kind: TokenKind::Str(s) }) = tokens.next() {
                    Some(s)
                } else {
                    None
                }
            }
            _ => panic!("Expected string token, got: {:?}", tokens.peek()),
        }
    }

    fn parse_object(tokens: &mut std::iter::Peekable<impl Iterator<Item = Token>>) -> Token {
        let mut object = std::collections::HashMap::new();

        // consume the opening '{'
        match tokens.peek() {
            Some(Token {
                kind: TokenKind::Punc('{'),
            }) => tokens.next(),
            _ => panic!("Unexpected token: {:?}", tokens.peek()),
        };

        loop {
            match tokens.peek() {
                Some(Token {
                    kind: TokenKind::Punc('}'),
                }) => {
                    // consume the closing '}'
                    tokens.next();
                    break;
                }
                Some(Token {
                    kind: TokenKind::Str(_),
                }) => {
                    loop {
                        let Some(key) = Self::parse_string(tokens) else {
                            panic!("Expected string key in object, got: {:?}", tokens.peek());
                        };
                        let Some(Token {
                            kind: TokenKind::Punc(':'),
                        }) = tokens.next()
                        else {
                            panic!("Expected ':' after key in object, got: {:?}", tokens.peek());
                        };
                        let value = Self::parse_value(tokens);
                        object.insert( key, value);

                        match tokens.peek() {
                            Some(Token {
                                kind: TokenKind::Punc(','),
                            }) => {
                                tokens.next(); // consume the comma and continue parsing the next key-value pair
                            }
                            Some(Token {
                                kind: TokenKind::Punc('}'),
                            }) => {
                                // consume the closing '}' and break out of the loop
                                tokens.next();
                                break;
                            }
                            _ => panic!(
                                "Expected ',' or '}}' after value in object, got: {:?}",
                                tokens.peek()
                            ),
                        }
                    }
                }
                _ => panic!("Unexpected token in object: {:?}", tokens.peek()),
            }
        }

        Token {
            kind: TokenKind::Object(object),
        }
    }
}
