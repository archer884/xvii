use std::borrow;

#[derive(Debug)]
pub struct ParseRomanError {
    kind: ParseRomanErrorKind,
    message: borrow::Cow<'static, str>,
}

#[derive(Debug)]
pub enum ParseRomanErrorKind {
    InvalidDigit(u8),
    OutOfRange(i32),
}

impl ParseRomanError {
    pub fn invalid_digit(digit: u8) -> ParseRomanError {
        ParseRomanError {
            kind: ParseRomanErrorKind::InvalidDigit(digit),
            message: borrow::Cow::Borrowed("Invalid digit"),
        }
    }

    pub fn out_of_range(n: i32) -> ParseRomanError {
        ParseRomanError {
            kind: ParseRomanErrorKind::OutOfRange(n),
            message: borrow::Cow::Borrowed("Value out of range (1...3999)"),
        }
    }
}
