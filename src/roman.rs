use std::fmt::{Display, Formatter, Result as FmtResult};
use std::{ops, str};

use crate::{Numeral, Result, Error};

/// A Roman number
///
/// Stores the value internally as a vector of Roman Numerals, and as a u32, to optimize usage.
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
        self.to_numerals().into_iter().map(Numeral::to_lowercase).collect()
    }

    /// Returns uppercase string representation of the Roman numeral
    pub fn to_uppercase(self) -> String {
        self.to_numerals().into_iter().map(Numeral::to_uppercase).collect()
    }

    /// Returns vector of numerals representing the roman numeral
    pub fn to_numerals(self) -> Vec<Numeral> {
        let val = self.0;
        if val == 0 {
            return vec![];
        }
        let mut num = val;
        [Numeral::M, Numeral::C, Numeral::X, Numeral::I]
            .iter()
            .fold(vec![], |mut acc, &step| {
                let num_step: u32 = step.into();
                if num >= num_step {
                    let current = num - (num % num_step);
                    self.partial_convert(current, step).into_iter().rev().for_each(|n| {
                        acc.push(n);
                    });
                    num -= current;
                }
                acc
            })
    }

    fn partial_convert(self, mut num: u32, base: Numeral) -> Vec<Numeral> {
        let mut numerals = vec![];
        let step: u32 = base.into();

        while num > 0 {
            match Numeral::from_int(num) {
                Ok(n) => {
                    numerals.push(n);
                    num = 0;
                },
                Err(_) => {
                    match Numeral::from_int(num + step) {
                        Ok(n) => {
                            numerals.push(n);
                            numerals.push(base);
                            num = 0;
                        },
                        Err(_) => {
                            numerals.push(base);
                            num -= step;
                        }
                    }
                }
            }
        }
        numerals
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
        let mut acc = s.chars().try_fold(Accumulator { val: 0, prev: None }, |mut acc, c| {
            let numeral = Numeral::from_char(c)?;
            let current = *numeral;
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
