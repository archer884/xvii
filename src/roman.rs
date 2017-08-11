use error::ParseRomanError;
use std::fmt;
use std::ops;
use std::str;
use unit::RomanUnitIterator;

mod ladder {
    pub static UPPER: &'static [(&'static str, i32)] = &[("M", 1000),
                                                         ("CM", 900),
                                                         ("D", 500),
                                                         ("CD", 400),
                                                         ("C", 100),
                                                         ("XC", 90),
                                                         ("L", 50),
                                                         ("XL", 40),
                                                         ("X", 10),
                                                         ("IX", 9),
                                                         ("V", 5),
                                                         ("IV", 4),
                                                         ("I", 1)];

    pub static LOWER: &'static [(&'static str, i32)] = &[("m", 1000),
                                                         ("cm", 900),
                                                         ("d", 500),
                                                         ("cd", 400),
                                                         ("c", 100),
                                                         ("xc", 90),
                                                         ("l", 50),
                                                         ("xl", 40),
                                                         ("x", 10),
                                                         ("ix", 9),
                                                         ("v", 5),
                                                         ("iv", 4),
                                                         ("i", 1)];
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
/// A Roman numeral.
///
/// This struct stores the value of a numeral as an `i32` but provides
/// for Roman-style formatting.
pub struct Roman(i32);

impl Roman {
    /// Creates a `Roman` value based on an `i32`.
    ///
    /// This function will return `None` if the value supplied is outside the
    /// acceptable range of `1...3999`, because numbers outside that range
    /// cannot be appropriately formatted using the seven standard numerals.
    pub fn from(n: i32) -> Option<Roman> {
        match n {
            n @ 1...3999 => Some(Roman(n)),
            _ => None,
        }
    }

    /// Creates a `Roman` value based on an `i32`.
    ///
    /// This function will return any `i32` wrapped in a `Roman` newtype
    /// without bothering to check its range, regardless of how unprintable
    /// it is.
    ///
    /// > Note: being "unprintable" is not memory unsafe and will not panic.
    pub fn from_unchecked(n: i32) -> Roman {
        Roman(n)
    }

    /// Formats a `Roman` value as an uppercase Roman numeral.
    pub fn to_uppercase(&self) -> String {
        let mut current = self.0;
        let mut buf = String::new();
        for &(unit, value) in ladder::UPPER.iter() {
            while current >= value {
                buf.push_str(unit);
                current -= value;
            }
        }
        buf
    }

    /// Formats a `Roman` value as a lowercase Roman numeral.
    pub fn to_lowercase(&self) -> String {
        let mut current = self.0;
        let mut buf = String::new();
        for &(unit, value) in ladder::LOWER.iter() {
            while current >= value {
                buf.push_str(unit);
                current -= value;
            }
        }
        buf
    }
}

impl AsRef<i32> for Roman {
    fn as_ref(&self) -> &i32 {
        &self.0
    }
}

impl ops::Deref for Roman {
    type Target = i32;

    fn deref(&self) -> &i32 {
        &self.0
    }
}

impl str::FromStr for Roman {
    type Err = ParseRomanError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        RomanUnitIterator::new(s)
            .sum::<Result<i32, ParseRomanError>>()
            .and_then(|n| Roman::from(n).ok_or_else(|| ParseRomanError::out_of_range(n)))
    }
}

impl fmt::Display for Roman {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.to_uppercase())
    }
}

#[cfg(test)]
mod tests {
    use roman::Roman;

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
    fn max_value_equals_3999() {
        assert_eq!("MMMCMXCIX", Roman(3999).to_string());
    }
}
