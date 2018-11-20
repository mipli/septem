use std::fmt::{Display, Formatter, Result as FmtResult};
use std::{ops, str};

use crate::{Digit, Result, Error};

/// A Roman number
///
/// Stores the value internally a u32
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub struct Roman(u32);

impl Roman {
    /// Creates a Roman numeral for any value that implements `Into<u32>`. Requires value to be
    /// greater than 0, and less than 4000.
    ///
    /// # Examples
    /// ```rust
    /// # use septem::prelude::*;
    /// # use septem::*;
    ///
    /// let sept: Roman = Roman::from(7u32).unwrap();
    /// assert_eq!("VII", sept.to_string());
    /// ```
    ///
    /// Returns `Roman` , or an `septem::Error
    pub fn from<T: Into<u32>>(val: T) -> Result<Self> {
        let val = val.into();
        if val == 0 || val > 3999 {
            return Err(Error::OutOfRange(val));
        }
        Ok(Roman(val))
    }

    /// Creates a Roman numeral for any value that implements `Into<u32>`. Does not to any range
    /// checking, so can result in weird numerals.
    ///
    /// # Examples
    /// ```rust
    /// # use septem::prelude::*;
    /// # use septem::*;
    ///
    /// let sept: Roman = Roman::from_unchecked(5032u32);
    /// assert_eq!("MMMMMXXXII", sept.to_string());
    /// ```
    ///
    /// Returns `Roman`
    pub fn from_unchecked<T: Into<u32>>(val: T) -> Self {
        Roman(val.into())
    }

    /// Returns lowercase string representation of the Roman numeral
    pub fn to_lowercase(self) -> String {
        self.to_digits().into_iter().map(Digit::to_lowercase).collect()
    }

    /// Returns uppercase string representation of the Roman numeral
    pub fn to_uppercase(self) -> String {
        self.to_digits().into_iter().map(Digit::to_uppercase).collect()
    }

    /// Returns vector of digits representing the roman numeral
    pub fn to_digits(self) -> Vec<Digit> {
        enum Pair {
            Single(Digit),
            Double((Digit, Digit))
        }
        let val = self.0;
        if val == 0 {
            return vec![];
        }
        let mut num = val;
        let pairs = [
            (1000, Pair::Single(Digit::M)),
            (900,  Pair::Double((Digit::C, Digit::M))),
            (500,  Pair::Single(Digit::D)),
            (400,  Pair::Double((Digit::C, Digit::D))),
            (100,  Pair::Single(Digit::C)),
            (90,   Pair::Double((Digit::X, Digit::C))),
            (50,   Pair::Single(Digit::L)),
            (40,   Pair::Double((Digit::X, Digit::L))),
            (10,   Pair::Single(Digit::X)),
            (9,    Pair::Double((Digit::I, Digit::X))),
            (5,    Pair::Single(Digit::V)),
            (4,    Pair::Double((Digit::I, Digit::V))),
            (1,    Pair::Single(Digit::I))
        ];

        let mut acc = vec![];
        for (val, pair) in pairs.iter() {
            while num >= *val {
                match pair {
                    Pair::Single(s) => acc.push(*s),
                    Pair::Double((a, b)) => {
                        acc.push(*a);
                        acc.push(*b);
                    }
                }
                num -= val;
            }
        }
        acc
    }
}

unsafe impl Send for Roman {}
unsafe impl Sync for Roman {}

impl ops::Deref for Roman {
    type Target = u32;

    /// Returns the integer representation of the Roman numeral
    fn deref(&self) -> &u32 {
        &self.0
    }
}

/// Support dereferencing to u32
///
/// # Examples
/// ```rust
/// # use septem::prelude::*;
/// # use septem::*;
///
/// let sept: Roman = Roman::from(7u32).unwrap();
/// let r: &u32 = &sept;
/// assert_eq!(&7, r);
/// ```
impl AsRef<u32> for Roman {
    fn as_ref(&self) -> &u32 {
        &self.0
    }
}

impl str::FromStr for Roman {
    type Err = Error;

    /// Creates a Roman numeral from a string representing the roman numerals
    ///
    /// # Examples
    /// ```rust
    /// # use septem::prelude::*;
    /// # use septem::*;
    ///
    /// let sept: Roman = "VII".parse().unwrap();
    /// assert_eq!(7, *sept);
    /// ```
    ///
    /// Returns `Roman` , or an `septem::Error`
    fn from_str(s: &str) -> std::result::Result<Self, Error> {
        use std::cmp::Ordering::{Equal, Less, Greater};
        struct Accumulator {
            val: u32,
            prev: Option<u32>
        }
        let mut acc = s.bytes().try_fold(Accumulator { val: 0, prev: None }, |mut acc, c| {
            let digit = Digit::from_byte(c)?;
            let current = *digit;
            if acc.prev.is_none() {
                acc.prev = Some(current);
                return Ok(acc);
            }
            let prev = acc.prev.unwrap();
            match current.cmp(&prev) {
                Equal => {
                    acc.val += prev;
                },
                Less => {
                    acc.val += prev;
                    acc.prev = Some(current);
                },
                Greater => {
                    acc.val += current - prev;
                    acc.prev = None;
                }
            }
            Ok(acc)
        })?;
        if let Some(prev) = acc.prev {
            acc.val += prev;
        }
        Ok(Roman(acc.val))
    }
}

impl Display for Roman {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        f.write_str(&self.to_uppercase())
    }
}
