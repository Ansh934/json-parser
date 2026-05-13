use crate::constants::*;
use crate::error::LexerError;
use crate::token::*;

pub(crate) struct Lexer {
    pub tokens: Vec<Token>,
}

impl Lexer {
    fn is_whitespace(c: char) -> bool {
        c == SPACE || c == NEWLINE || c == TAB || c == CARRIAGE_RETURN
    }

    fn is_punc(c: char) -> bool {
        c == LEFT_BRACE
            || c == RIGHT_BRACE
            || c == LEFT_BRACKET
            || c == RIGHT_BRACKET
            || c == COLON
            || c == COMMA
    }

    fn is_keyword(s: &str) -> bool {
        s == TRUE || s == FALSE || s == NULL
    }

    fn is_keyword_start(c: char) -> bool {
        c == TRUE.chars().next().unwrap()
            || c == FALSE.chars().next().unwrap()
            || c == NULL.chars().next().unwrap()
    }

    fn is_number_start(c: char) -> bool {
        // number can only start with a digit or a minus sign
        c.is_digit(DECIMAL_BASE) || c == MINUS
    }

    fn is_number_end(c: char) -> bool {
        Self::is_whitespace(c) || c == COMMA || c == RIGHT_BRACE || c == RIGHT_BRACKET
    }

    fn read_whitespace(
        input: &mut std::iter::Peekable<impl Iterator<Item = char>>,
    ) -> Result<(), LexerError> {
        Self::read_while(input, Self::is_whitespace)?;
        Ok(())
    }

    fn read_string(
        input: &mut std::iter::Peekable<impl Iterator<Item = char>>,
    ) -> Result<Token, LexerError> {
        let mut result = String::new();

        // consume the opening quote
        // dbg!(&result);
        match input.peek() {
            Some(&QUOTE) => input.next(), // consume the opening quote and continue
            Some(&c) => return Err(LexerError::UnexpectedCharacter(c)),
            None => return Err(LexerError::UnexpectedEndOfInput),
        };
        // dbg!(&result);
        loop {
            match input.peek() {
                Some(&QUOTE) => {
                    input.next(); // consume the closing quote
                    break;
                } // empty string case
                Some(&BACKSLASH) => {
                    input.next(); // consume the escape character
                    match input.peek() {
                        Some(&QUOTE) | Some(&BACKSLASH) | Some(&SLASH) => {
                            result.push(input.next().unwrap()); // consume the escaped character and add it to the result
                        }
                        Some(&LOWECASE_B) => {
                            result.push(BACKSPACE);
                            input.next(); // consume the escaped character
                        }
                        Some(&LOWECASE_F) => {
                            result.push(FORM_FEED);
                            input.next(); // consume the escaped character
                        }
                        Some(&LOWECASE_N) => {
                            result.push(NEWLINE);
                            input.next(); // consume the escaped character
                        }
                        Some(&LOWECASE_R) => {
                            result.push(CARRIAGE_RETURN);
                            input.next(); // consume the escaped character
                        }
                        Some(&LOWECASE_T) => {
                            result.push(TAB);
                            input.next(); // consume the escaped character
                        }
                        Some(&LOWECASE_U) => {
                            input.next();
                            // a single hex digit can be (digit || A-F || a-f)
                            let hex_digits =
                                Self::read_while(input, |c| c.is_digit(HEXADECIMAL_BASE))?;
                            if hex_digits.len() != UNICODE_ESCAPE_LENGTH {
                                return Err(LexerError::InvalidUnicodeEscapeSequence(hex_digits));
                            }
                            result.push_str(UNICODE_ESCAPE_PREFIX);
                            result.push_str(&hex_digits);
                        }
                        Some(&c) => return Err(LexerError::InvalidEscapeSequence(c)),
                        None => return Err(LexerError::UnterminatedString),
                    };
                }
                Some(&c) => {
                    result.push(c);
                    input.next();
                }
                None => return Err(LexerError::UnterminatedString),
            };
        }
        // dbg!(&result);
        Ok(Token::Str(result))
    }

    fn read_number(
        input: &mut std::iter::Peekable<impl Iterator<Item = char>>,
    ) -> Result<Token, LexerError> {
        let mut num_str = String::new();

        // step 1 handle negative numbers
        match input.peek() {
            Some(&MINUS) => {
                num_str.push(MINUS);
                input.next(); // consume the '-'
            }
            Some(&c) if c.is_digit(DECIMAL_BASE) => (), // continue to handle the number in the next steps
            Some(&c) => return Err(LexerError::UnexpectedCharacter(c)),
            None => return Err(LexerError::UnexpectedEndOfInput),
        }

        // step 2 handle the integer part (including leading zero case)
        match input.peek() {
            Some(&ZERO) => {
                num_str.push(ZERO);
                input.next(); // consume the '0'
            }
            Some(&c) if c.is_digit(DECIMAL_BASE) => {
                num_str.push_str(&Self::read_while(input, |c| c.is_digit(DECIMAL_BASE))?);
            }
            Some(&c) => return Err(LexerError::UnexpectedCharacter(c)),
            None => return Err(LexerError::UnexpectedEndOfInput),
        }

        //step 3 handle the fractional part
        match input.peek() {
            Some(&DECIMAL_POINT) => {
                num_str.push(DECIMAL_POINT);
                input.next(); // consume the '.' and add it to the num_str
                match input.peek() {
                    Some(&c) if c.is_digit(DECIMAL_BASE) => {
                        num_str.push_str(&Self::read_while(input, |c| c.is_digit(DECIMAL_BASE))?)
                    }
                    Some(&c) => return Err(LexerError::UnexpectedCharacter(c)),
                    None => return Err(LexerError::UnexpectedEndOfInput),
                }
            }
            Some(&EXPONENT_LOWER) | Some(&EXPONENT_UPPER) => (), // handle exponent part in the next step
            Some(&c) if Self::is_number_end(c) => (), // end of number, do nothing and let the next token handle it
            Some(&c) => return Err(LexerError::UnexpectedCharacter(c)),
            None => return Err(LexerError::UnexpectedEndOfInput),
        }

        // step 4 handle the exponent part
        match input.peek() {
            Some(&EXPONENT_LOWER) | Some(&EXPONENT_UPPER) => {
                num_str.push(input.next().unwrap()); // consume the 'e' or 'E'
                match input.peek() {
                    Some(&PLUS) | Some(&MINUS) => {
                        num_str.push(input.next().unwrap()); // consume the '+' or '-' and add it to the num_str
                    }
                    Some(&c) if c.is_digit(DECIMAL_BASE) => (), // continue to handle the exponent in the next step
                    Some(&c) => return Err(LexerError::UnexpectedCharacter(c)),
                    None => return Err(LexerError::UnexpectedEndOfInput),
                }
                match input.peek() {
                    Some(&c) if c.is_digit(DECIMAL_BASE) => {
                        num_str.push_str(&Self::read_while(input, |c| c.is_digit(DECIMAL_BASE))?)
                    }
                    Some(&c) => return Err(LexerError::UnexpectedCharacter(c)),
                    None => return Err(LexerError::UnexpectedEndOfInput),
                }
            }
            Some(&c) if Self::is_number_end(c) => (), // end of number, do nothing and let the next token handle it
            Some(&c) => return Err(LexerError::UnexpectedCharacter(c)),
            None => return Err(LexerError::UnexpectedEndOfInput),
        }

        let num = num_str
            .parse::<f64>()
            .map_err(|_| LexerError::InvalidNumber(num_str))?;
        Ok(Token::Num(num))
    }

    fn read_keyword(
        input: &mut std::iter::Peekable<impl Iterator<Item = char>>,
    ) -> Result<Token, LexerError> {
        let keyword = Self::read_while(input, |c| c.is_alphabetic())?;
        if !Self::is_keyword(&keyword) {
            return Err(LexerError::UnexpectedKeyword(keyword));
        }
        let token = match keyword.as_str() {
            TRUE => Token::True,
            FALSE => Token::False,
            NULL => Token::Null,
            _ => Err(LexerError::UnexpectedKeyword(keyword))?,
        };
        Ok(token)
    }

    fn read_punc(
        input: &mut std::iter::Peekable<impl Iterator<Item = char>>,
    ) -> Result<Token, LexerError> {
        match input.peek() {
            Some(&LEFT_BRACE) => {
                input.next(); // consume the '{'
                return Ok(Token::LeftBrace);
            }
            Some(&RIGHT_BRACE) => {
                input.next(); // consume the '}'
                return Ok(Token::RightBrace);
            }
            Some(&LEFT_BRACKET) => {
                input.next(); // consume the '['
                return Ok(Token::LeftBracket);
            }   
            Some(&RIGHT_BRACKET) => {
                input.next(); // consume the ']'
                return Ok(Token::RightBracket);
            }
            Some(&COLON) => {
                input.next(); // consume the ':'
                return Ok(Token::Colon);
            }
            Some(&COMMA) => {
                input.next(); // consume the ','
                return Ok(Token::Comma);
            }
            Some(&c) => return Err(LexerError::UnexpectedCharacter(c)),
            None => return Err(LexerError::UnexpectedEndOfInput),
        }
    }

    /// read while condition is true and input is not empty
    /// returns the string of characters read that satisfy the condition
    /// returns Error if input is empty before reading any character
    fn read_while(
        input: &mut std::iter::Peekable<impl Iterator<Item = char>>,
        condition: impl Fn(char) -> bool,
    ) -> Result<String, LexerError> {
        if input.peek().is_none() {
            return Err(LexerError::UnexpectedEndOfInput);
        }
        let mut result = String::new();
        while let Some(&c) = input.peek() {
            if condition(c) {
                result.push(c);
                input.next();
            } else {
                break;
            }
        }
        Ok(result)
    }

    fn read_next(
        input: &mut std::iter::Peekable<impl Iterator<Item = char>>,
    ) -> Result<Token, LexerError> {
        match input.peek() {
            Some(&QUOTE) => return Self::read_string(input),
            Some(&c) if Self::is_number_start(c) => return Self::read_number(input),
            Some(&c) if Self::is_keyword_start(c) => return Self::read_keyword(input),
            Some(&c) if Self::is_punc(c) => return Self::read_punc(input),
            Some(&c) => return Err(LexerError::UnexpectedCharacter(c)),
            None => return Err(LexerError::UnexpectedEndOfInput),
            // None => Ok(Token::Null), // return null token to signify end of input
        }
    }

    pub(crate) fn tokenize(input: impl Iterator<Item = char>) -> Result<Self, LexerError> {
        let mut input = input.peekable();
        let mut tokens = Vec::new();
        while input.peek().is_some() {
            // dbg!(&tokens);
            Self::read_whitespace(&mut input)?;
            if input.peek().is_some() {
                let token = Self::read_next(&mut input)?;
                // println!("Read token: {:?}", token);
                tokens.push(token);
            }
        }
        Ok(Self { tokens })
    }
}
