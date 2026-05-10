#[derive(Debug)]
pub(crate) enum TokenKind {
    Punc(char),
    Num(f64),
    Str(String),
    True,
    False,
    Null,
}

#[derive(Debug)]
pub(crate) struct Token {
    kind: TokenKind,
}

pub(crate) struct Lexer {
    pub tokens: Vec<Token>,
}

type InputStream = std::iter::Peekable<dyn Iterator<Item = char>>;

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

    // read while condition is true and input is not empty
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

    fn read_whitespace(input: &mut std::iter::Peekable<impl Iterator<Item = char>>) {
        Self::read_while(input, Self::is_whitespace);
    }

    fn read_next(input: &mut std::iter::Peekable<impl Iterator<Item = char>>) -> Option<Token> {
        //read whitespace
        Self::read_whitespace(input);

        // get peek char and if it is none return none
        let Some(c) = input.peek() else {
            return None;
        };

        if c == &'"' {
            return Some(Self::read_string(input));
        };

        if c.is_digit(10) {
            return Some(Self::read_number(input));
        };

        if Self::is_keyword_start(*c) {
            return Some(Self::read_keyword(input));
        }

        if Self::is_punc(*c) {
            return Some(Self::read_punc(input));
        };

        // werent able to parse a token, so we return
        // None
        panic!("Unexpected character: {}", c);
    }

    pub(crate) fn tokenize(input: impl Iterator<Item = char>) -> Self {
        let mut input = input.peekable();
        let mut tokens = Vec::new();
        while let Some(token) = Self::read_next(&mut input) {
            tokens.push(token);
        }
        Self { tokens }
    }

    fn read_string(input: &mut std::iter::Peekable<impl Iterator<Item = char>>) -> Token {
        let mut result = String::new();
        input.next(); // consume the opening quote
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
            panic!("Unterminated string");
        }
        Token {
            kind: TokenKind::Str(result),
        }
    }

    fn read_number(input: &mut std::iter::Peekable<impl Iterator<Item = char>>) -> Token {
        let num_str = Self::read_while(input, |c| c.is_digit(10) || c == '.');
        let num = num_str.parse::<f64>().unwrap(); // panic if we fail to parse a number, since we should have only digits and dots
        Token {
            kind: TokenKind::Num(num),
        }
    }

    fn read_keyword(input: &mut std::iter::Peekable<impl Iterator<Item = char>>) -> Token {
        let keyword = Self::read_while(input, |c| c.is_alphabetic());
        if !Self::is_keyword(&keyword) {
            panic!("Unexpected keyword: {}", keyword);
        }
        let kind = match keyword.as_str() {
            "true" => TokenKind::True,
            "false" => TokenKind::False,
            "null" => TokenKind::Null,
            _ => unreachable!(),
        };
        Token { kind }
    }

    fn read_punc(input: &mut std::iter::Peekable<impl Iterator<Item = char>>) -> Token {
        let c = input.next().unwrap();
        Token {
            kind: TokenKind::Punc(c),
        }
    }
}
