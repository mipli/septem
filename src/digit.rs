use std::fmt::{Display, Formatter, Result as FmtResult};
use std::ops;
use crate::{Result, Error};

/// Representation of a roman digit
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Digit {
    I,
    V,
    X,
    L,
    C,
    D,
    M
}

impl Digit {
    /// Tries to converts a value that implements `Into<u32>` into a single roman digit
    ///
    /// # Examples
    /// ```rust
    /// # use septem::prelude::*;
    /// # use septem::*;
    ///
    /// let v: Digit = Digit::from_int(5u8).unwrap();
    /// assert_eq!(Digit::V, v);
    /// ```
    ///
    /// Returns `Digit` , or an `septem::Error`
    pub fn from_int<T: Into<u32>>(num: T) -> Result<Digit> {
        let num: u32 = num.into();
        use self::Digit::*;
        match num {
            1 => Ok(I),
            5 => Ok(V),
            10 => Ok(X),
            50 => Ok(L),
            100 => Ok(C),
            500 => Ok(D),
            1000 => Ok(M),
            _ => Err(Error::InvalidNumber(num))
        }
    }
}

impl Digit {
    /// Tries to converts a char into a single roman digit
    ///
    /// # Examples
    /// ```rust
    /// # use septem::prelude::*;
    /// # use septem::*;
    ///
    /// let v: Digit = Digit::from_char('v').unwrap();
    /// assert_eq!(Digit::V, v);
    /// ```
    ///
    /// Returns `Digit` , or an `septem::Error`
    pub fn from_char(c: char) -> Result<Digit> {
        use self::Digit::*;
        match c.to_uppercase().next() {
            Some('I') => Ok(I),
            Some('V') => Ok(V),
            Some('X') => Ok(X),
            Some('L') => Ok(L),
            Some('C') => Ok(C),
            Some('D') => Ok(D),
            Some('M') => Ok(M),
            _ => Err(Error::InvalidDigit(c))
        }
    }

    pub fn to_lowercase(self) -> char {
        use self::Digit::*;
        match self {
            I => 'i',
            V => 'v',
            X => 'x',
            L => 'l',
            C => 'c',
            D => 'd',
            M => 'm'
        }
    }

    pub fn to_uppercase(self) -> char {
        use self::Digit::*;
        match self {
            I => 'I',
            V => 'V',
            X => 'X',
            L => 'L',
            C => 'C',
            D => 'D',
            M => 'M'
        }
    }
}

unsafe impl Send for Digit {}
unsafe impl Sync for Digit {}

impl From<Digit> for u32 {
    /// Converts from Digit to u32
    fn from(digit: Digit) -> u32 {
        *digit
    }
}

impl From<&Digit> for char {
    /// Converts from &Digit to char
    fn from(digit: &Digit) -> char {
        digit.to_uppercase()
    }
}

impl Display for Digit {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", char::from(self))
    }
}

impl ops::Deref for Digit {
    type Target = u32;

    /// Returns from &Digit to u32
    fn deref(&self) -> &u32 {
        use self::Digit::*;
        match *self {
            I => &1,
            V => &5,
            X => &10,
            L => &50,
            C => &100,
            D => &500,
            M => &1000
        }
    }
}
