// TODO
// - [ ] Handle escape characters in strings
// - [ ] Handle numbers better
// - [ ] Implement Iterator for Lexer so we can iterate over tokens
// - [o] Handle errors better instead of panicking
// - [x] Add SourceLocation to tokens for better error reporting

use crate::error::LexerError;
use crate::token::*;

pub(crate) struct Lexer {
    pub tokens: Vec<Token>,
}

impl Lexer {
    fn is_whitespace(c: char) -> bool {
        c == ' ' || c == '\n' || c == '\t' || c == '\r'
    }

    fn is_punc(c: char) -> bool {
        c == '{' || c == '}' || c == '[' || c == ']' || c == ':' || c == ','
    }

    fn is_keyword(s: &str) -> bool {
        s == "true" || s == "false" || s == "null"
    }

    fn is_keyword_start(c: char) -> bool {
        c == 't' || c == 'f' || c == 'n'
    }

    fn read_whitespace(input: &mut std::iter::Peekable<impl Iterator<Item = char>>) {
        Self::read_while(input, Self::is_whitespace);
    }

    fn read_string(
        input: &mut std::iter::Peekable<impl Iterator<Item = char>>,
    ) -> Result<Token, LexerError> {
        let mut result = String::new();

        // consume the opening quote
        match input.peek() {
            Some('"') => input.next(), // consume the opening quote and continue
            Some(&c) => return Err(LexerError::UnexpectedCharacter(c)),
            None => return Err(LexerError::UnexpectedEndOfInput),
        };

        let mut found_closing_quote = false;
        while let Some(&c) = input.peek() {
            if c == '"' {
                input.next(); // consume the closing quote
                found_closing_quote = true;
                break;
            } else {
                result.push(c);
                input.next();
            }
        }
        if !found_closing_quote {
            return Err(LexerError::UnterminatedString);
        }
        Ok(Token::Str(result))
    }

    fn read_number(
        input: &mut std::iter::Peekable<impl Iterator<Item = char>>,
    ) -> Result<Token, LexerError> {
        let num_str = Self::read_while(input, |c| c.is_digit(10) || c == '.');
        let num = num_str
            .parse::<f64>()
            .map_err(|_| LexerError::InvalidNumber(num_str))?;
        Ok(Token::Num(num))
    }

    fn read_keyword(
        input: &mut std::iter::Peekable<impl Iterator<Item = char>>,
    ) -> Result<Token, LexerError> {
        let keyword = Self::read_while(input, |c| c.is_alphabetic());
        if !Self::is_keyword(&keyword) {
            return Err(LexerError::UnexpectedKeyword(keyword));
        }
        let token = match keyword.as_str() {
            "true" => Token::True,
            "false" => Token::False,
            "null" => Token::Null,
            _ => Err(LexerError::UnexpectedKeyword(keyword))?,
        };
        Ok(token)
    }

    fn read_punc(
        input: &mut std::iter::Peekable<impl Iterator<Item = char>>,
    ) -> Result<Token, LexerError> {
        let Some(&c) = input.peek() else {
            return Err(LexerError::UnexpectedEndOfInput);
        };
        if !Self::is_punc(c) {
            return Err(LexerError::UnexpectedCharacter(c));
        }
        input.next(); // consume the punctuation character
        Ok(Token::Punc(c))
    }

    /// read while condition is true and input is not empty
    /// returns the string of characters read that satisfy the condition
    /// returns empty string if the first character does not satisfy the condition or if the input is empty
    // no result enum is needed
    fn read_while(
        input: &mut std::iter::Peekable<impl Iterator<Item = char>>,
        condition: impl Fn(char) -> bool,
    ) -> String {
        let mut result = String::new();
        while let Some(&c) = input.peek() {
            if condition(c) {
                result.push(c);
                input.next();
            } else {
                break;
            }
        }
        result
    }

    fn read_next(
        input: &mut std::iter::Peekable<impl Iterator<Item = char>>,
    ) -> Result<Token, LexerError> {
        Self::read_whitespace(input);
        match input.peek() {
            Some('"') => return Self::read_string(input),
            Some(&c) if c.is_digit(10) => return Self::read_number(input),
            Some(&c) if Self::is_keyword_start(c) => return Self::read_keyword(input),
            Some(&c) if Self::is_punc(c) => return Self::read_punc(input),
            Some(&c) => return Err(LexerError::UnexpectedCharacter(c)),
            None => return Err(LexerError::UnexpectedEndOfInput),
        }
    }

    pub(crate) fn tokenize(input: impl Iterator<Item = char>) -> Result<Self, LexerError> {
        let mut input = input.peekable();
        let mut tokens = Vec::new();
        while input.peek().is_some() {
            tokens.push(Self::read_next(&mut input)?);
        }
        Ok(Self { tokens })
    }
}
