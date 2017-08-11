use std::error;
use std::fmt;

use std::borrow::Cow;

#[derive(Debug)]
/// An error in parsing a Roman numeral.
pub struct ParseRomanError {
    kind: ParseRomanErrorKind,
    message: Cow<'static, str>,
}

#[derive(Debug)]
pub enum ParseRomanErrorKind {
    /// An invalid digit was encountered when parsing.
    InvalidDigit(u8),

    /// Parsing was successful, but the resulting value is out of range.
    OutOfRange(i32),
}

impl ParseRomanError {
    pub fn invalid_digit(digit: u8) -> ParseRomanError {
        ParseRomanError {
            kind: ParseRomanErrorKind::InvalidDigit(digit),
            message: Cow::from("Invalid digit"),
        }
    }

    pub fn out_of_range(n: i32) -> ParseRomanError {
        ParseRomanError {
            kind: ParseRomanErrorKind::OutOfRange(n),
            message: Cow::from("Value out of range (1...3999)"),
        }
    }
}

impl fmt::Display for ParseRomanError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use std::error::Error;

        match self.kind {
            ParseRomanErrorKind::InvalidDigit(digit) => write!(f, "{}: {}", self.description(), (digit as char)),
            ParseRomanErrorKind::OutOfRange(value) => write!(f, "{}: {}", self.description(), value),
        }
    }
}

impl error::Error for ParseRomanError {
    fn description(&self) -> &str {
        match self.kind {
            ParseRomanErrorKind::InvalidDigit(_) => "Parser encountered an invalid digit",
            ParseRomanErrorKind::OutOfRange(_) => "Resulting value was out of range",
        }
    }
}
