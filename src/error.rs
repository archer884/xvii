use std::borrow::Cow;
use std::error;
use std::fmt;

pub(crate) type Result<T> = ::std::result::Result<T, RomanError>;

#[derive(Debug)]
/// An error in parsing a Roman numeral.
pub struct ParseRomanError {
    kind: RomanErrorKind,
    message: Cow<'static, str>,
}

#[derive(Debug)]
/// An error in parsing a Roman numeral.
pub struct RomanError {
    kind: RomanErrorKind,
    message: Cow<'static, str>,
}

#[derive(Debug)]
pub enum RomanErrorKind {
    /// An invalid digit was encountered when parsing.
    InvalidDigit(u8),

    /// Parsing was successful, but the resulting value is out of range.
    OutOfRange(i32),
}

impl RomanError {
    pub fn invalid_digit(digit: u8) -> RomanError {
        RomanError {
            kind: RomanErrorKind::InvalidDigit(digit),
            message: Cow::from("Invalid digit"),
        }
    }

    pub fn out_of_range(n: i32) -> RomanError {
        RomanError {
            kind: RomanErrorKind::OutOfRange(n),
            message: Cow::from("Value out of range (1...4999)"),
        }
    }
}

impl fmt::Display for RomanError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use std::error::Error;

        match self.kind {
            RomanErrorKind::InvalidDigit(digit) => {
                write!(f, "{}: {}", self.description(), (digit as char))
            }
            
            RomanErrorKind::OutOfRange(value) => {
                write!(f, "{}: {}", self.description(), value)
            }
        }
    }
}

impl error::Error for RomanError {
    fn description(&self) -> &str {
        match self.kind {
            RomanErrorKind::InvalidDigit(_) => "Parser encountered an invalid digit",
            RomanErrorKind::OutOfRange(_) => "Resulting value was out of range",
        }
    }
}
