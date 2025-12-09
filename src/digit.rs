use crate::{Error, Result};
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::ops;

/// Representation of a roman digit
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Digit {
    I,
    V,
    X,
    L,
    C,
    D,
    M,

    #[cfg(feature = "archaic")]
    OneThousandOld, // ↀ
    #[cfg(feature = "archaic")]
    FiveThousand, // ↁ
    #[cfg(feature = "archaic")]
    TenThousand, // ↂ
    #[cfg(feature = "archaic")]
    FiftyThousand, // ↇ
    #[cfg(feature = "archaic")]
    HundredThousand, // ↈ
}

impl Digit {
    /// Converts any positive integer into a vector of Roman digits.
    ///
    /// # Examples
    /// ```rust
    /// # use septem::prelude::*;
    /// # use septem::*;
    ///
    /// let three = Digit::from_int(3u8).unwrap();
    /// assert_eq!(three, vec![Digit::I, Digit::I, Digit::I]);
    ///
    /// let eight = Digit::from_int(8u8).unwrap();
    /// assert_eq!(eight, vec![Digit::V, Digit::I, Digit::I, Digit::I]);
    ///
    /// let nine = Digit::from_int(9u8).unwrap();
    /// assert_eq!(nine, vec![Digit::I, Digit::X]);
    ///
    /// let nineteen_ninety_four = Digit::from_int(1994u32).unwrap();
    /// assert_eq!(
    ///     nineteen_ninety_four,
    ///     vec![
    ///         Digit::M, Digit::C, Digit::M, // 1900
    ///         Digit::X, Digit::C,           // 90
    ///         Digit::I, Digit::V            // 4
    ///     ]
    /// );
    ///
    /// assert!(Digit::from_int(0u8).is_err(), "zero is invalid");
    /// ```
    ///
    /// Returns `Vec<Digit>`, or an `septem::Error` if the number is zero or too large.
    pub fn from_int<T>(num: T) -> Result<Vec<Digit>>
    where
        T: Into<u32> + Copy + PartialOrd + From<u8>,
    {
        let mut n: u32 = num.into();
        if n == 0 {
            return Err(Error::InvalidNumber(n));
        }

        use Digit::*;

        const TABLE: &[(u32, &[Digit])] = &[
            (1000, &[M]),
            (900, &[C, M]),
            (500, &[D]),
            (400, &[C, D]),
            (100, &[C]),
            (90, &[X, C]),
            (50, &[L]),
            (40, &[X, L]),
            (10, &[X]),
            (9, &[I, X]),
            (5, &[V]),
            (4, &[I, V]),
            (1, &[I]),
        ];

        let mut result = Vec::with_capacity(15);

        for &(value, digits) in TABLE {
            if n == 0 {
                break;
            }

            let count = n / value;
            match digits {
                &[a] => result.extend(std::iter::repeat(a).take(count as usize)),
                &[a, b] => (0..count).for_each(|_| {
                    result.push(a);
                    result.push(b);
                }),
                _ => unreachable!(),
            }

            n %= value;
        }

        Ok(result)
    }

    /// Returns the numeric value of this Roman digit as any type that implements `From<u32>`.
    ///
    /// # Examples
    /// ```rust
    /// # use septem::prelude::*;
    /// # use septem::*;
    ///
    /// let v: u32 = Digit::V.value();
    /// let x: u64 = Digit::X.value();
    /// let f: f64 = Digit::L.value();
    ///
    /// assert_eq!(v, 5);
    /// assert_eq!(x, 10);
    /// assert_eq!(f, 50.0);
    /// ```
    pub fn value<T>(&self) -> T
    where
        T: From<u32>,
    {
        let v = match self {
            Digit::I => 1,
            Digit::V => 5,
            Digit::X => 10,
            Digit::L => 50,
            Digit::C => 100,
            Digit::D => 500,
            Digit::M => 1000,

            #[cfg(feature = "archaic")]
            Digit::OneThousandOld => 1000,
            #[cfg(feature = "archaic")]
            Digit::FiveThousand => 5000,
            #[cfg(feature = "archaic")]
            Digit::TenThousand => 10000,
            #[cfg(feature = "archaic")]
            Digit::FiftyThousand => 50000,
            #[cfg(feature = "archaic")]
            Digit::HundredThousand => 100000,
        };
        T::from(v)
    }

    /// Computes the numeric value of a Roman numeral sequence (e.g. `XIV` → `14`).
    ///
    /// # Examples
    /// ```rust
    /// # use septem::prelude::*;
    /// # use septem::*;
    ///
    /// let digits = vec![Digit::X, Digit::I, Digit::I];
    /// assert_eq!(Digit::value_of::<u32>(&digits), 12);
    ///
    /// let digits = vec![Digit::X, Digit::I, Digit::V];
    /// assert_eq!(Digit::value_of::<u32>(&digits), 14);
    ///
    /// let digits = vec![Digit::I, Digit::X];
    /// assert_eq!(Digit::value_of::<u64>(&digits), 9);
    /// ```
    pub fn value_of<T>(digits: &[Digit]) -> T
    where
        T: From<u32>
            + Copy
            + std::ops::Add<Output = T>
            + std::ops::Sub<Output = T>
            + PartialOrd
            + Default,
    {
        let mut total = T::default();
        let mut i = 0;

        while i < digits.len() {
            let curr = digits[i].value::<T>();
            if i + 1 < digits.len() {
                let next = digits[i + 1].value::<T>();
                if curr < next {
                    total = total + (next - curr);
                    i += 2;
                    continue;
                }
            }
            total = total + curr;
            i += 1;
        }

        total
    }
}

impl Digit {
    /// Tries to convert a character into a single Roman digit.
    ///
    /// Supports:
    /// - ASCII letters: `'I', 'V', 'X', 'L', 'C', 'D', 'M'` (case-insensitive)
    /// - Unicode Number Forms: `'Ⅰ'..'Ⅿ'` (U+2160–U+216F) and `'ⅰ'..'ⅿ'` (U+2170–U+217F)
    /// - (Optional) Archaic forms `'ↀ'..'ↈ'` when `feature = "archaic"` is enabled
    ///
    /// Returns a vector of `Digit` representing the decomposed Roman numeral if applicable.
    ///
    /// # Examples
    /// ```rust
    /// # use septem::prelude::*;
    /// # use septem::*;
    ///
    /// // ASCII lowercase
    /// let v = Digit::from_char('v').unwrap();
    /// assert_eq!(v, vec![Digit::V]);
    ///
    /// // ASCII uppercase
    /// let x = Digit::from_char('X').unwrap();
    /// assert_eq!(x, vec![Digit::X]);
    ///
    /// // Unicode uppercase
    /// let viii = Digit::from_char('Ⅷ').unwrap();
    /// assert_eq!(viii, vec![Digit::V, Digit::I, Digit::I, Digit::I]);
    ///
    /// // Unicode lowercase
    /// let iv = Digit::from_char('ⅳ').unwrap();
    /// assert_eq!(iv, vec![Digit::I, Digit::V]);
    ///
    /// // Invalid character
    /// let err = Digit::from_char('A');
    /// assert!(err.is_err());
    /// ```
    ///
    /// # Archaic Numerals
    /// ```rust
    /// # #[cfg(feature = "archaic")]
    /// # {
    /// # use septem::prelude::*;
    /// # use septem::*;
    ///
    /// let thousand = Digit::from_char('ↀ').unwrap();
    /// assert_eq!(thousand, vec![Digit::OneThousandOld]);
    ///
    /// let five_thousand = Digit::from_char('ↁ').unwrap();
    /// assert_eq!(five_thousand, vec![Digit::FiveThousand]);
    ///
    /// let ten_thousand = Digit::from_char('ↂ').unwrap();
    /// assert_eq!(ten_thousand, vec![Digit::TenThousand]);
    ///
    /// let fifty_thousand = Digit::from_char('ↇ').unwrap();
    /// assert_eq!(fifty_thousand, vec![Digit::FiftyThousand]);
    ///
    /// let hundred_thousand = Digit::from_char('ↈ').unwrap();
    /// assert_eq!(hundred_thousand, vec![Digit::HundredThousand]);
    /// # }
    /// ```
    ///
    /// Returns `Vec<Digit>` or an [`septem::Error::InvalidDigit`].
    pub fn from_char(c: char) -> Result<Vec<Digit>> {
        use self::Digit::*;

        let result = match c {
            // Single Roman numerals (ASCII + Unicode uppercase/lowercase)
            'Ⅰ' | 'ⅰ' | 'I' | 'i' => vec![I],
            'Ⅱ' | 'ⅱ' => vec![I, I],
            'Ⅲ' | 'ⅲ' => vec![I, I, I],
            'Ⅳ' | 'ⅳ' => vec![I, V],
            'Ⅴ' | 'ⅴ' | 'V' | 'v' => vec![V],
            'Ⅵ' | 'ⅵ' => vec![V, I],
            'Ⅶ' | 'ⅶ' => vec![V, I, I],
            'Ⅷ' | 'ⅷ' => vec![V, I, I, I],
            'Ⅸ' | 'ⅸ' => vec![I, X],
            'Ⅹ' | 'ⅹ' | 'X' | 'x' => vec![X],
            'Ⅺ' | 'ⅺ' => vec![X, I],
            'Ⅻ' | 'ⅻ' => vec![X, I, I],
            'Ⅼ' | 'ⅼ' | 'L' | 'l' => vec![L],
            'Ⅽ' | 'ⅽ' | 'C' | 'c' => vec![C],
            'Ⅾ' | 'ⅾ' | 'D' | 'd' => vec![D],
            'Ⅿ' | 'ⅿ' | 'M' | 'm' => vec![M],

            // Optional archaic numerals
            #[cfg(feature = "archaic")]
            'ↀ' => vec![OneThousandOld],
            #[cfg(feature = "archaic")]
            'ↁ' => vec![FiveThousand],
            #[cfg(feature = "archaic")]
            'ↂ' => vec![TenThousand],
            #[cfg(feature = "archaic")]
            'ↇ' => vec![FiftyThousand],
            #[cfg(feature = "archaic")]
            'ↈ' => vec![HundredThousand],

            _ => return Err(Error::InvalidDigit(c)),
        };

        Ok(result)
    }

    /// Tries to converts a byte into a single roman digit
    ///
    /// # Examples
    /// ```rust
    /// # use septem::prelude::*;
    /// # use septem::*;
    ///
    /// let v: Digit = Digit::from_byte(b'v').unwrap();
    /// assert_eq!(Digit::V, v);
    /// ```
    ///
    /// Returns `Digit` , or an `septem::Error`
    pub fn from_byte(b: u8) -> Result<Digit> {
        use self::Digit::*;
        match b {
            b'I' | b'i' => Ok(I),
            b'V' | b'v' => Ok(V),
            b'X' | b'x' => Ok(X),
            b'L' | b'l' => Ok(L),
            b'C' | b'c' => Ok(C),
            b'D' | b'd' => Ok(D),
            b'M' | b'm' => Ok(M),
            _ => Err(Error::InvalidDigit(b.into())),
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
            M => 'm',
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
            M => 'M',
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

impl<'a> From<&'a Digit> for char {
    /// Converts from &Digit to char
    fn from(digit: &'a Digit) -> char {
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
            M => &1000,
        }
    }
}
