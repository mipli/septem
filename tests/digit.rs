#[cfg(test)]
mod tests {
    use septem::{Digit, Error};

    #[test]
    fn from_int_valid() {
        let n = Digit::from_int(5u8);
        assert!(n.is_ok());
        assert_eq!(Digit::V, n.unwrap());
    }

    #[test]
    fn from_int_invalid() {
        match Digit::from_int(3u32) {
            Err(Error::InvalidNumber(num)) => assert_eq!(3, num),
            _ => assert!(false)
        }
    }

    #[test]
    fn to_int() {
        let i: u32 = Digit::V.into();
        assert_eq!(5u32, i);
    }

    #[test]
    fn digit_display() {
        assert_eq!('V', (&Digit::V).into());
    }
}
