use std::error;
use std::fmt;
use std::result;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    InvalidDigit(char),
    InvalidNumber(u64),
    OutOfRange(u32),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Error::*;
        use std::error::Error;

        match *self {
            InvalidDigit(digit) => write!(f, "{}: {}", self.description(), digit),
            InvalidNumber(number) => write!(f, "{}: {}", self.description(), number),
            OutOfRange(value) => write!(f, "{}: {}", self.description(), value),
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
