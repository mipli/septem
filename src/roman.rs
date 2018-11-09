use std::fmt::{Display, Formatter, Result as FmtResult};
use std::cmp::{Ordering};
use std::{ops, str};

use crate::{Numeral, Result, Error};

#[derive(Debug, Clone)]
pub struct Roman {
    numerals: Vec<Numeral>,
    value: u32
}

impl Roman {
    pub fn numerals(&self) -> &Vec<Numeral> {
        &self.numerals
    }

    pub fn iter(&self) -> impl Iterator<Item = &Numeral> {
        self.numerals.iter()
    }

    pub fn from<T: Into<u32>>(val: T) -> Result<Self> {
        let val = val.into();
        if val == 0 || val > 3999 {
            return Err(Error::OutOfRange(val));
        }
        Ok(Roman::from_unchecked(val))
    }

    pub fn from_unchecked<T: Into<u32>>(val: T) -> Self {
        let val = val.into();
        if val == 0 {
            return Roman {
                value: 0,
                numerals: vec![]
            };
        }
        let mut num = val;
        let numerals = [Numeral::M, Numeral::C, Numeral::X, Numeral::I]
            .iter()
            .fold(vec![], |mut acc, &step| {
                let num_step: u32 = step.into();
                if num >= num_step {
                    let current = num - (num % num_step);
                    acc.extend(Roman::partial_convert(current, step));
                    num -= current;
                }
                acc
            });

        Roman {
            value: val,
            numerals
        }
    }

    fn partial_convert(mut num: u32, base: Numeral) -> Vec<Numeral> {
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
        numerals.reverse();
        numerals
    }

    pub fn to_lowercase(&self) -> String {
        self.to_string().to_lowercase()
    }
}

unsafe impl Send for Roman {}
unsafe impl Sync for Roman {}

impl ops::Deref for Roman {
    type Target = u32;

    fn deref(&self) -> &u32 {
        &self.value
    }
}

impl str::FromStr for Roman {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Error> {
        use std::cmp::Ordering::{Equal, Less, Greater};
        struct Accumulator {
            numerals: Vec<Numeral>,
            val: u32,
            prev: Option<u32>
        }
        let mut acc = s.chars().try_fold(Accumulator { numerals: vec![], val: 0, prev: None }, |mut acc, c| {
            let numeral = Numeral::from_char(c)?;
            acc.numerals.push(numeral);
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

        Ok(Roman {
            numerals: acc.numerals,
            value: acc.val
        })
    }
}

impl Display for Roman {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        let nums = self.numerals
            .iter()
            .map(|n| char::from(n))
            .collect::<String>();
        write!(f, "{}", nums)
    }
}

impl PartialEq for Roman {
    fn eq(&self, other: &Roman) -> bool {
        self.value == other.value
    }
}

impl PartialOrd for Roman {
    fn partial_cmp(&self, other: &Roman) -> Option<Ordering> {
        Some(self.value.cmp(&other.value))
    }
}
