use crate::{unit::RomanUnitIterator, Error, Result};
use std::fmt::{self, Display};
use std::str::FromStr;

struct LadderEntry {
    upper: &'static str,
    lower: &'static str,
    value: u16,
}

static LADDER: &[LadderEntry] = &[
    LadderEntry {
        upper: "M",
        lower: "m",
        value: 1000,
    },
    LadderEntry {
        upper: "CM",
        lower: "cm",
        value: 900,
    },
    LadderEntry {
        upper: "D",
        lower: "d",
        value: 500,
    },
    LadderEntry {
        upper: "CD",
        lower: "cd",
        value: 400,
    },
    LadderEntry {
        upper: "C",
        lower: "c",
        value: 100,
    },
    LadderEntry {
        upper: "XC",
        lower: "xc",
        value: 90,
    },
    LadderEntry {
        upper: "L",
        lower: "l",
        value: 50,
    },
    LadderEntry {
        upper: "XL",
        lower: "xl",
        value: 40,
    },
    LadderEntry {
        upper: "X",
        lower: "x",
        value: 10,
    },
    LadderEntry {
        upper: "IX",
        lower: "ix",
        value: 9,
    },
    LadderEntry {
        upper: "V",
        lower: "v",
        value: 5,
    },
    LadderEntry {
        upper: "IV",
        lower: "iv",
        value: 4,
    },
    LadderEntry {
        upper: "I",
        lower: "i",
        value: 1,
    },
];

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
/// A Roman numeral.
///
/// This struct stores the value of a numeral as an `u16` but provides
/// for Roman-style formatting.
pub struct Roman(u16);

impl Roman {
    /// Creates a `Roman` value based on a `u16`.
    ///
    /// This function will return `None` if the value supplied is outside the
    /// acceptable range of `1...4999`, because numbers outside that range
    /// cannot be appropriately formatted using the seven standard numerals.
    pub fn new(n: u16) -> Option<Roman> {
        match n {
            n @ 1..=4999 => Some(Roman(n)),
            _ => None,
        }
    }

    /// Creates a `Roman` value based on a `u16`.
    ///
    /// This function will return any `u16` wrapped in a `Roman` newtype
    /// without bothering to check its range, regardless of how unprintable
    /// it is.
    ///
    /// > Note: being "unprintable" is not memory unsafe and will not panic.
    pub fn new_unchecked(n: u16) -> Roman {
        Roman(n)
    }

    /// Formats a `Roman` value as an uppercase Roman numeral.
    pub fn to_uppercase(self) -> String {
        let mut current = self.0;
        let mut buf = String::new();

        for entry in LADDER {
            while current >= entry.value {
                current -= entry.value;
                buf += entry.upper;
            }
        }

        buf
    }

    /// Formats a `Roman` value as a lowercase Roman numeral.
    pub fn to_lowercase(self) -> String {
        let mut current = self.0;
        let mut buf = String::new();

        for entry in LADDER {
            while current >= entry.value {
                current -= entry.value;
                buf += entry.lower;
            }
        }

        buf
    }

    pub fn format(&self, style: Style) -> RomanFormatter {
        RomanFormatter {
            style,
            value: self.0,
        }
    }

    pub fn into_inner(self) -> u16 {
        self.0
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Style {
    Lower,
    Upper,
}

#[derive(Debug, Copy, Clone)]
pub struct RomanFormatter {
    style: Style,
    value: u16,
}

impl Display for RomanFormatter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut current = self.value;

        for entry in LADDER {
            while current >= entry.value {
                match self.style {
                    Style::Lower => f.write_str(entry.lower)?,
                    Style::Upper => f.write_str(entry.upper)?,
                }
                current -= entry.value;
            }
        }

        Ok(())
    }
}

impl FromStr for Roman {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match RomanUnitIterator::new(s).sum::<Result<i32>>()? {
            sum @ 1..=4999 => Ok(Roman::new_unchecked(sum as u16)),
            sum => Err(Error::OutOfRange(sum)),
        }
    }
}

impl Display for Roman {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.format(Style::Upper))
    }
}

#[cfg(test)]
mod tests {
    use super::Roman;

    #[test]
    fn mcmlxxxiv_equals_1984() {
        assert_eq!("MCMLXXXIV", Roman(1984).to_string());
    }

    #[test]
    fn mmdxxix_equals_2529() {
        assert_eq!("MMDXXIX", Roman(2529).to_string());
    }

    #[test]
    fn mmcmxcix_equals_2999() {
        assert_eq!("MMCMXCIX", Roman(2999).to_string());
    }

    #[test]
    fn mmmcmxcix_value_equals_3999() {
        assert_eq!("MMMCMXCIX", Roman(3999).to_string());
    }

    #[test]
    fn max_value_equals_4999() {
        assert_eq!("MMMMCMXCIX", Roman(4999).to_string());
    }

    #[test]
    fn mmmmcmxcix_parses_as_4999() {
        let result: Roman = "MMMMCMXCIX".parse().unwrap();
        assert_eq!(4999, result.into_inner());
    }
}
