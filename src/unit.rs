use error::ParseRomanError;
use std::str;

/// Iterates "units" of a Roman numeral.
///
/// I have arbitrarily decided that a "unit" of a Roman numeral is any sequence
/// of characters which does not shrink in value. `IX` is one unit. `XII` is two
/// units. The first has a value of `9`, while the second is two values: `[10, 2]`.
/// My theory is that this will allow me to calculate the value of a Roman numeral
/// by reading from left to right just once.
pub struct RomanUnitIterator<'a> {
    bytes: str::Bytes<'a>,
    last: Option<i32>,
}

impl<'a> RomanUnitIterator<'a> {
    pub fn new(s: &'a str) -> RomanUnitIterator<'a> {
        RomanUnitIterator {
            bytes: s.bytes(),
            last: None,
        }
    }
}

impl<'a> Iterator for RomanUnitIterator<'a> {
    type Item = Result<i32, ParseRomanError>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.bytes.next() {
                Some(x) => {
                    let partial = match to_digit(x) {
                        Ok(x) => x,
                        Err(e) => return Some(Err(e)),
                    };

                    match self.last {
                        Some(prior) => {
                            if partial > prior {
                                self.last = None;
                                return Some(Ok(partial - prior));
                            } else {
                                self.last = Some(partial + prior);
                            }
                        }

                        None => self.last = Some(partial),
                    }
                }

                None => return self.last.take().map(|x| Ok(x)),
            }
        }
    }
}

fn to_digit(c: u8) -> Result<i32, ParseRomanError> {
    use std::ascii::AsciiExt;
    match c.to_ascii_lowercase() {
        b'm' => Ok(1000),
        b'd' => Ok(500),
        b'c' => Ok(100),
        b'l' => Ok(50),
        b'x' => Ok(10),
        b'v' => Ok(5),
        b'i' => Ok(1),

        _ => Err(ParseRomanError::invalid_digit(c)),
    }
}

#[cfg(test)]
mod tests {
    use roman::Roman;
    use unit::RomanUnitIterator;

    #[test]
    fn i_equals_1() {
        assert_eq!(1, *"i".parse::<Roman>().unwrap());
        assert_eq!(1, *"I".parse::<Roman>().unwrap());
    }

    #[test]
    fn i_equals_sequence_1() {
        assert_eq!(1, RomanUnitIterator::new("i").next().unwrap().unwrap());
        assert_eq!(1, RomanUnitIterator::new("I").next().unwrap().unwrap());
    }

    #[test]
    fn ix_equals_9() {
        assert_eq!(9, *"ix".parse::<Roman>().unwrap());
    }

    #[test]
    fn iiiiix_equals_5() {
        // Yes, I know this is stupid, but this is how units are meant to work.
        assert_eq!(5, *"iiiiix".parse::<Roman>().unwrap());
    }
}
