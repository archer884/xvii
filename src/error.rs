use core::fmt::{self, Display};

/// An error in parsing a Roman numeral.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Error {
    /// Encountered an invalid digit while parsing.
    InvalidDigit(u8),

    /// Value out of range.
    OutOfRange(u16),

    /// Value is way out of range (> 65536).
    Overflow,
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::InvalidDigit(digit) => {
                write!(f, "Parser encountered an invalid digit: {}", *digit as char)
            }
            Error::OutOfRange(value) => write!(f, "Value out of range: {}", value),
            Error::Overflow => f.write_str("Value out of range"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}
