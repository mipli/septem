use std::fmt::{Display, Formatter, Result as FmtResult};
use std::ops;
use crate::{Result, Error};

/// Representation of a roman numeral
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Numeral {
    I,
    V,
    X,
    L,
    C,
    D,
    M
}

impl Numeral {
    /// Tries to converts a value that implements `Into<u32>` into a single roman numeral
    ///
    /// # Examples
    /// ```rust
    /// # use septem::prelude::*;
    /// # use septem::*;
    ///
    /// let v: Numeral = Numeral::from_int(5u8).unwrap();
    /// assert_eq!(Numeral::V, v);
    /// ```
    ///
    /// Returns `Numeral` , or an `septem::Error`
    pub fn from_int<T: Into<u32>>(num: T) -> Result<Numeral> {
        let num: u32 = num.into();
        use self::Numeral::*;
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

impl Numeral {
    /// Tries to converts a char into a single roman numeral
    ///
    /// # Examples
    /// ```rust
    /// # use septem::prelude::*;
    /// # use septem::*;
    ///
    /// let v: Numeral = Numeral::from_char('v').unwrap();
    /// assert_eq!(Numeral::V, v);
    /// ```
    ///
    /// Returns `Numeral` , or an `septem::Error`
    pub fn from_char(c: char) -> Result<Numeral> {
        use self::Numeral::*;
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

    pub fn to_lowercase(&self) -> char {
        use self::Numeral::*;
        match *self {
            I => 'i',
            V => 'v',
            X => 'x',
            L => 'l',
            C => 'c',
            D => 'd',
            M => 'm'
        }
    }

    pub fn to_uppercase(&self) -> char {
        use self::Numeral::*;
        match *self {
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

unsafe impl Send for Numeral {}
unsafe impl Sync for Numeral {}

impl From<Numeral> for u32 {
    /// Converts from Numeral to u32
    fn from(numeral: Numeral) -> u32 {
        *numeral
    }
}

impl From<&Numeral> for char {
    /// Converts from &Numeral to char
    fn from(numeral: &Numeral) -> char {
        numeral.to_uppercase()
    }
}

impl Display for Numeral {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", char::from(self))
    }
}

impl ops::Deref for Numeral {
    type Target = u32;

    /// Returns from &Numeral to u32
    fn deref(&self) -> &u32 {
        use self::Numeral::*;
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
