use std::error;
use std::fmt;
use std::result;

pub(crate) type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
/// An error in parsing a Roman numeral.
pub enum Error {
    /// Encountered an invalid digit while parsing.
    InvalidDigit(u8),

    /// Value out of range.
    OutOfRange(i32),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Error::*;
        use std::error::Error;

        match *self {
            InvalidDigit(digit) => write!(f, "{}: {}", self.description(), digit as char),
            OutOfRange(value) => write!(f, "{}: {}", self.description(), value),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        use self::Error::*;

        match *self {
            InvalidDigit(_) => "Parser encountered an invalid digit",
            OutOfRange(_) => "Resulting value was out of range",
        }
    }
}
