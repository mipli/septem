use std::error;
use std::fmt;
use std::result;

pub type Result<T> = result::Result<T, Error>;

/// Error for Roman Numeral parsing
#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    /// A char that is not a valid roman numeral
    InvalidDigit(char),
    /// A number that cannot be converted to a single Roman Numeral
    InvalidNumber(u32),
    /// Value is out of range
    OutOfRange(u32),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Error::*;

        match *self {
            InvalidDigit(digit) => write!(f, "InvalidDigit: {digit}"),
            InvalidNumber(number) => write!(f, "InvalidNumber: {number}"),
            OutOfRange(value) => write!(f, "OutOfRange: {value}"),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        use self::Error::*;

        match *self {
            InvalidDigit(_) => "Encountered an invalid digit",
            InvalidNumber(_) => "Cannot convert number to single roman digit",
            OutOfRange(_) => "Roman numeral is out of range",
        }
    }
}
