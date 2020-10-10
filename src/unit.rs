use crate::{Error, Result};
use std::str;

/// Accumulates the value of a single numeral "unit".
///
/// `qty` represents the number of times the numeral has appeared in the unit, while `num` represents
/// the numeric value of the numeral. Obviously, the final value of the unit is evaluated by
/// multiplying these two.
#[derive(Default)]
struct Accumulator {
    qty: u16,
    val: u16,
}

impl Accumulator {
    fn new(val: u16) -> Self {
        Accumulator { qty: 1, val }
    }

    fn push(mut self, val: u16) -> PushResult {
        use std::cmp::Ordering::*;

        match self.val.cmp(&val) {
            Equal => {
                self.qty += 1;
                PushResult::Partial(self)
            }

            Less => PushResult::Complete(val - self.value(), None),
            Greater => PushResult::Complete(self.value(), Some(Accumulator::new(val))),
        }
    }

    fn value(&self) -> u16 {
        self.qty * self.val
    }
}

/// The result of a push onto an accumulator.
///
/// Adding an additional instance of the original unit onto an accumulator emits a partial result
/// because the accumulator may have an indefinite number of such instances. `I`, `II`, `III`, `IIIIIII`
/// and so forth are all valid contents for an accumulator. However, changing the unit value
/// will cause the accumulator to emit a complete result, signifying that a value should be
/// produced by the iterator and a new accumulator created.
enum PushResult {
    Partial(Accumulator),
    Complete(u16, Option<Accumulator>),
}

/// Iterates "units" of a Roman numeral.
///
/// I have arbitrarily decided that a "unit" of a Roman numeral is any sequence
/// of characters which does not shrink in value. `IX` is one unit. `XII` is two
/// units. The first has a value of `9`, while the second is two values: `[10, 2]`.
/// My theory is that this will allow me to calculate the value of a Roman numeral
/// by reading from left to right just once.
pub struct RomanUnitIterator<'a> {
    bytes: str::Bytes<'a>,
    acc: Option<Accumulator>,
}

impl<'a> RomanUnitIterator<'a> {
    pub fn new(s: &'a str) -> RomanUnitIterator<'a> {
        RomanUnitIterator {
            bytes: s.bytes(),
            acc: None,
        }
    }
}

impl<'a> Iterator for RomanUnitIterator<'a> {
    type Item = Result<u16>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let value = match self.bytes.next() {
                None => return self.acc.take().map(|acc| Ok(acc.value())),
                Some(u) => match to_digit(u) {
                    Ok(u) => u,
                    Err(e) => return Some(Err(e)),
                },
            };

            match self.acc.take() {
                None => self.acc = Some(Accumulator::new(value)),
                Some(acc) => match acc.push(value) {
                    PushResult::Partial(acc) => self.acc = Some(acc),
                    PushResult::Complete(n, acc) => {
                        self.acc = acc;
                        return Some(Ok(n));
                    }
                },
            }
        }
    }
}

fn to_digit(u: u8) -> Result<u16> {
    match u.to_ascii_lowercase() {
        b'm' => Ok(1000),
        b'd' => Ok(500),
        b'c' => Ok(100),
        b'l' => Ok(50),
        b'x' => Ok(10),
        b'v' => Ok(5),
        b'i' => Ok(1),

        _ => Err(Error::InvalidDigit(u)),
    }
}

#[cfg(test)]
mod tests {
    use super::RomanUnitIterator;
    use crate::Roman;

    #[test]
    fn to_digit_works() {
        let digits = b"mDcLxVi";
        assert!(digits.iter().all(|&d| super::to_digit(d).is_ok()));

        let digits = b"aBeFgH";
        assert!(digits.iter().all(|&d| super::to_digit(d).is_err()));
    }

    #[test]
    fn i_equals_1() {
        assert_eq!(1, "i".parse::<Roman>().unwrap().get());
        assert_eq!(1, "I".parse::<Roman>().unwrap().get());
    }

    #[test]
    fn i_equals_sequence_1() {
        assert_eq!(1, RomanUnitIterator::new("i").next().unwrap().unwrap());
        assert_eq!(1, RomanUnitIterator::new("I").next().unwrap().unwrap());
    }

    #[test]
    fn ix_equals_9() {
        assert_eq!(9, "ix".parse::<Roman>().unwrap().get());
    }

    #[test]
    fn iiiiix_equals_5() {
        // Yes, I know this is stupid, but this is how units are meant to work.
        assert_eq!(5, "iiiiix".parse::<Roman>().unwrap().get());
    }
}
