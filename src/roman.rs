mod ladder;

use crate::{unit::RomanUnitIterator, Error, Result};
use core::{
    fmt::{self, Display},
    num::NonZeroU16,
    str::FromStr,
};

/// A Roman numeral.
///
/// This struct stores the value of a numeral as an [`NonZeroU16`] but provides
/// for Roman-style formatting.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Roman(NonZeroU16);

impl Roman {
    /// Creates a `Roman` value based on a [`u16`].
    ///
    /// This function will return `None` if the value supplied is outside the
    /// acceptable range of `1..=4999`, because numbers outside that range
    /// cannot be appropriately formatted using the seven standard numerals.
    pub fn new(n: u16) -> Option<Roman> {
        match n {
            n if n <= 4999 => NonZeroU16::new(n).map(Roman),
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
    #[cfg(feature = "std")]
    #[cfg_attr(docsrs, doc(cfg(feature = "std")))]
    pub fn to_uppercase(self) -> String {
        let mut current = self.0.get();
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
    #[cfg(feature = "std")]
    #[cfg_attr(docsrs, doc(cfg(feature = "std")))]
    pub fn to_lowercase(self) -> String {
        let mut current = self.0.get();
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

    /// Returns value of this `Roman` numeral.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// let roman = xvii::Roman::new(42).unwrap();
    /// assert_eq!(roman.value(), 42);
    /// ```
    pub fn value(self) -> u16 {
        self.0.get()
    }

    /// Returns the inner value.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// let roman = xvii::Roman::new(42).unwrap();
    /// assert_eq!(roman.into_inner(), std::num::NonZeroU16::new(42).unwrap());
    /// ```
    pub fn into_inner(self) -> NonZeroU16 {
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
/// This struct is created by [`format`](Roman::format) method.
#[derive(Debug, Copy, Clone)]
pub struct RomanFormatter {
    style: Style,
    value: NonZeroU16,
}

impl Display for RomanFormatter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut current = self.value.get();

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
        let sum = RomanUnitIterator::new(s)
            .try_fold(0, |acc, r| r?.checked_add(acc).ok_or(Error::Overflow))?;
        Roman::new(sum).ok_or(Error::OutOfRange(sum))
    }
}

impl Display for Roman {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.format(Style::Upper).fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use crate::Error;

    use super::Roman;

    #[test]
    fn mcmlxxxiv_equals_1984() {
        assert_eq!("MCMLXXXIV", Roman::new(1984).unwrap().to_string());
    }

    #[test]
    fn mmdxxix_equals_2529() {
        assert_eq!("MMDXXIX", Roman::new(2529).unwrap().to_string());
    }

    #[test]
    fn mmcmxcix_equals_2999() {
        assert_eq!("MMCMXCIX", Roman::new(2999).unwrap().to_string());
    }

    #[test]
    fn mmmcmxcix_value_equals_3999() {
        assert_eq!("MMMCMXCIX", Roman::new(3999).unwrap().to_string());
    }

    #[test]
    fn max_value_equals_4999() {
        assert_eq!("MMMMCMXCIX", Roman::new(4999).unwrap().to_string());
    }

    #[test]
    fn mmmmcmxcix_parses_as_4999() {
        let result: Roman = "MMMMCMXCIX".parse().unwrap();
        assert_eq!(4999, result.value());
    }

    #[test]
    fn overflow() {
        assert_eq!(
            Err(Error::Overflow),
            "MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMCM".parse::<Roman>()
        );

        assert_eq!(
            Err(Error::Overflow),
            "CMCMCMCMCMCMCMCMCMCMCMCMCMCMCMCMCMCMCMCMCMCMCMCMCMCMCMCMCMCMCMCMCMCMCMCMCMCMCMCMCMCMCMCMCMCMCMCMCMCMCMCMCMCMCMCMCMCMCMCMCMCMCMCMCMCMCMCMCMCMCMCMCMCMCMCMCMCMCMCMCMCMCMCMCMCMCMCMCMCMCMCMCMCMCMCM".parse::<Roman>()
        );

        assert_eq!(
            Err(Error::Overflow),
            "MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMCMCMCMCMCMCMCMCMCMCMCMCMCMCMCMCMCMCMCMCMCDCDCDCDCDCDCDCDCDCDCDCDCDCDCDCDCDCDCDCDCDCDCDCDCDCDCDCDCDCDCDCDCDCDCDCDCDCDCDCDCDCDCDCDCDCDCDCDCDCDCDCDCDCDCDCDCDCDCDCDCDCD".parse::<Roman>()
        );

        assert_eq!(
            Err(Error::Overflow),
            "MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM".parse::<Roman>()
        );

        assert_eq!(
            Err(Error::Overflow),
            "MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM".parse::<Roman>()
        );
    }
}
