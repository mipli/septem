#![feature(try_from)]
use std::convert::{TryFrom};
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Clone, Copy)]
enum Numeral {
    I,
    V,
    X,
    L,
    C,
    D,
    M
}

impl TryFrom<u32> for Numeral {
    type Error = ();

    fn try_from(num: u32) -> Result<Numeral, ()> {
        use crate::Numeral::*;
        match num {
            1 => Ok(I),
            5 => Ok(V),
            10 => Ok(X),
            50 => Ok(L),
            100 => Ok(C),
            500 => Ok(D),
            1000 => Ok(M),
            _ => Err(())
        }
    }
}

impl From<Numeral> for u32 {
    fn from(numeral: Numeral) -> u32 {
        use crate::Numeral::*;
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
        use crate::Numeral::*;
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


pub struct Roman {
    numerals: Vec<Numeral>
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

impl From<u32> for Roman {
    fn from(mut num: u32) -> Self {
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
            numerals
        }
    }
}

impl Roman {
    fn partial_convert(mut num: u32, base: Numeral) -> Vec<Numeral> {
        let mut numerals = vec![];
        let step: u32 = base.into();

        while num > 0 {
            match Numeral::try_from(num) {
                Ok(n) => {
                    numerals.push(n);
                    num = 0;
                },
                Err(_) => {
                    match Numeral::try_from(num + step) {
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
} 
