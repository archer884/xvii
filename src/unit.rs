use error::*;
use std::str;

/// Accumulates the value of a single numeral "unit."
///
/// qty represents the number of times the numeral has appeared in the unit, while num represents
/// the numeric value of the numeral. Obviously, the final value of the unit is evaluated by
/// multiplying these two.
#[derive(Default)]
struct Accumulator {
    qty: i32,
    val: i32,
}

impl Accumulator {
    fn new(val: i32) -> Self {
        Accumulator { qty: 1, val }
    }

    fn push(mut self, val: i32) -> PushResult {
        use std::cmp::Ordering::*;

        match self.val.cmp(&val) {
            Equal => {
                self.qty += 1;
                PushResult::Partial(self)
            }

            Less => PushResult::Complete(val - self.value(), None),
            Greater => PushResult::Complete(self.value(), Some(Accumulator::new(val)))
        }
    }

    fn value(&self) -> i32 {
        self.qty * self.val
    }
}

/// The result of a push onto an accumulator.
///
/// Adding an additional instance of the original unit onto an accumulator emits a partial result
/// because the accumulator may have an indefinite number of such instances. I, II, III, IIIIIII
/// and so forth are all valid contents for an accumulator. However, changing the unit value
/// will cause the accumulator to emit a complete result, signifying that a value should be
/// produced by the iterator and a new accumulator created.
enum PushResult {
    Partial(Accumulator),
    Complete(i32, Option<Accumulator>),
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
    type Item = Result<i32>;

    // This appears to be deeply nested. I haven't the foggiest how that happened. >.>
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.bytes.next() {
                // If there are no more bytes left, just check for a leftover accumulator.
                None => {
                    return self.acc.take().map(|acc| Ok(acc.value()));
                }

                Some(u) => {
                    // Return early if the next byte is invalid.
                    let value = match to_digit(u) {
                        Ok(u) => u,
                        Err(e) => return Some(Err(e)),
                    };

                    // Check for an existing accumulator.
                    match self.acc.take() {
                        // If we don't have one, make a new one with our new byte.
                        None => {
                            self.acc = Some(Accumulator::new(value));
                        }

                        // Apply the new byte to any existing accumulator.
                        Some(acc) => {
                            match acc.push(value) {
                                PushResult::Complete(n, acc) => {
                                    self.acc = acc;
                                    return Some(Ok(n));
                                }

                                PushResult::Partial(acc) => {
                                    self.acc = Some(acc);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn to_digit(c: u8) -> Result<i32> {
    use std::ascii::AsciiExt;
    match c.to_ascii_lowercase() {
        b'm' => Ok(1000),
        b'd' => Ok(500),
        b'c' => Ok(100),
        b'l' => Ok(50),
        b'x' => Ok(10),
        b'v' => Ok(5),
        b'i' => Ok(1),

        _ => Err(RomanError::invalid_digit(c)),
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
