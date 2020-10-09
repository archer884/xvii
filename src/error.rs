use std::error;
use std::fmt::{self, Display};

#[derive(Debug)]
/// An error in parsing a Roman numeral.
pub enum Error {
    /// Encountered an invalid digit while parsing.
    InvalidDigit(u8),

    /// Value out of range.
    OutOfRange(i32),
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::InvalidDigit(digit) => write!(
                f,
                "{}: {}",
                "Parser encountered an invalid digit", *digit as char
            ),
            Error::OutOfRange(value) => write!(f, "{}: {}", "Value out of range", value),
        }
    }
}

impl error::Error for Error {}
