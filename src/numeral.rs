use std::fmt::{Display, Formatter, Result as FmtResult};
use std::ops;
use crate::{Result, Error};

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
    pub fn from_int<T: Into<u64>>(num: T) -> Result<Numeral> {
        let num: u64 = num.into();
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
}

unsafe impl Send for Numeral {}
unsafe impl Sync for Numeral {}

impl From<Numeral> for u32 {
    fn from(numeral: Numeral) -> u32 {
        use self::Numeral::*;
        match numeral {
            I => 1,
            V => 5,
            X => 10,
            L => 50,
            C => 100,
            D => 500,
            M => 1000
        }
    }
}

impl From<&Numeral> for char {
    fn from(numeral: &Numeral) -> char {
        use self::Numeral::*;
        match *numeral {
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

impl Display for Numeral {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", char::from(self))
    }
}

impl ops::Deref for Numeral {
    type Target = u32;

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
