#[cfg(test)]
mod tests {
    use septem::{Numeral, Error};

    #[test]
    fn from_int_valid() {
        let n = Numeral::from_int(5u8);
        assert!(n.is_ok());
        assert_eq!(Numeral::V, n.unwrap());
    }

    #[test]
    fn from_int_invalid() {
        match Numeral::from_int(3u32) {
            Err(Error::InvalidNumber(num)) => assert_eq!(3, num),
            _ => assert!(false)
        }
    }

    #[test]
    fn to_int() {
        let i: u32 = Numeral::V.into();
        assert_eq!(5u32, i);
    }

    #[test]
    fn numeral_display() {
        assert_eq!('V', (&Numeral::V).into());
    }
}
