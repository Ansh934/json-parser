// whitespace characters
pub(crate) const SPACE: char = ' ';
pub(crate) const NEWLINE: char = '\n';
pub(crate) const TAB: char = '\t';
pub(crate) const CARRIAGE_RETURN: char = '\r';

// unicode escape sequence
pub(crate) const UNICODE_ESCAPE_PREFIX: &'static str = "\\u";
pub(crate) const UNICODE_ESCAPE_LENGTH: usize = 4;

// escape sequence characters
pub(crate) const LOWECASE_B : char = 'b';
pub(crate) const LOWECASE_F : char = 'f';
pub(crate) const LOWECASE_N : char = 'n';
pub(crate) const LOWECASE_R : char = 'r';
pub(crate) const LOWECASE_T : char = 't';
pub(crate) const LOWECASE_U : char = 'u';

// slashes
pub(crate) const SLASH: char = '/';
pub(crate) const BACKSLASH: char = '\\';

// control characters
pub(crate) const BACKSPACE: char = '\x08';
pub(crate) const FORM_FEED: char = '\x0C';

// punctuation characters
pub(crate) const LEFT_BRACE: char = '{';
pub(crate) const RIGHT_BRACE: char = '}';
pub(crate) const LEFT_BRACKET: char = '[';
pub(crate) const RIGHT_BRACKET: char = ']';
pub(crate) const COLON: char = ':';
pub(crate) const COMMA: char = ',';

// keywords
pub(crate) const TRUE: &'static str = "true";
pub(crate) const FALSE: &'static str = "false";
pub(crate) const NULL: &'static str = "null";

// string characters
pub(crate) const QUOTE: char = '"';

// number parsing characters
pub(crate) const EXPONENT_LOWER: char = 'e';
pub(crate) const EXPONENT_UPPER: char = 'E';
pub(crate) const MINUS: char = '-';
pub(crate) const PLUS: char = '+';
pub(crate) const DECIMAL_POINT: char = '.';
pub(crate) const ZERO: char = '0';

// bases
pub(crate) const DECIMAL_BASE: u32 = 10;
pub(crate) const HEXADECIMAL_BASE: u32 = 16;