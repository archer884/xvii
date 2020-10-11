mod ladder;

use crate::{unit::RomanUnitIterator, Error, Result};
use std::{
    fmt::{self, Display},
    str::FromStr,
};

/// A Roman numeral.
///
/// This struct stores the value of a numeral as an [`u16`] but provides
/// for Roman-style formatting.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Roman(u16);

impl Roman {
    /// Creates a `Roman` value based on a [`u16`].
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

    /// Formats a [`Roman`] value as an uppercase Roman numeral.
    ///
    /// ## Examples
    ///
    /// ```
    /// use xvii::Roman;
    /// assert_eq!(Roman::new(42).unwrap().to_uppercase(), "XLII");
    /// ```
    pub fn to_uppercase(self) -> String {
        let mut current = self.0;
        let mut buf = String::new();

        for entry in ladder::VALUES {
            while current >= entry.value {
                current -= entry.value;
                buf += entry.upper;
            }
        }

        buf
    }

    /// Formats a [`Roman`] value as a lowercase Roman numeral.
    ///
    /// ## Examples
    ///
    /// ```
    /// use xvii::Roman;
    /// assert_eq!(Roman::new(42).unwrap().to_lowercase(), "xlii");
    /// ```
    pub fn to_lowercase(self) -> String {
        let mut current = self.0;
        let mut buf = String::new();

        for entry in ladder::VALUES {
            while current >= entry.value {
                current -= entry.value;
                buf += entry.lower;
            }
        }

        buf
    }

    /// Returns a [`RomanFormatter`] which lazily formats a `self` value as a lowercase or uppercase Roman numeral depending of `style`.
    ///
    /// ## Examples
    ///
    /// ```
    /// use xvii::{Roman, Style};
    ///
    /// let value = Roman::new(12).unwrap();
    /// assert_eq!(format!("{}", value.format(Style::Upper)), "XII");
    /// assert_eq!(value.format(Style::Lower).to_string(), "xii"); // `format!("{}")` and `.to_string()` are the same thing
    /// ```
    pub fn format(&self, style: Style) -> RomanFormatter {
        RomanFormatter {
            style,
            value: self.0,
        }
    }

    /// Returns the inner value.
    pub fn get(self) -> u16 {
        self.0
    }
}

/// Style of formatting — lowercase or uppercase.
#[derive(Debug, Copy, Clone)]
pub enum Style {
    /// Lowercase formatting. E.g.: `xvii`.
    Lower,
    /// Uppercase formatting. E.g.: `XVII`.
    Upper,
}

/// Lazy roman formatter.
///
/// This struct is created by [`format`] method.
#[derive(Debug, Copy, Clone)]
pub struct RomanFormatter {
    style: Style,
    value: u16,
}

impl Display for RomanFormatter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut current = self.value;

        for entry in ladder::VALUES {
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
            sum @ 1..=4999 => Ok(Roman(sum as u16)),
            sum => Err(Error::OutOfRange(sum)),
        }
    }
}

impl Display for Roman {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.format(Style::Upper).fmt(f)
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
        assert_eq!(4999, result.get());
    }
}
