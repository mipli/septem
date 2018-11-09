#[cfg(test)]
mod tests {
    use septem::prelude::*;
    use septem::{Roman, Numeral, Error};

    #[test]
    fn from_valid() {
        use self::Numeral::*;

        let r = Roman::from(532u32);
        assert!(r.is_ok());
        let r = r.unwrap();
        assert_eq!(&vec![D, X, X, X, I, I], r.numerals());
        assert_eq!(532, *r);
    }

    #[test]
    fn parse_str() {
        use self::Numeral::*;

        let r: Roman = "DXXIX".parse().unwrap();
        assert_eq!(&vec![D, X, X, I, X], r.numerals());
        assert_eq!(529, *r);
    }

    #[test]
    fn from_str_valid() {
        use self::Numeral::*;

        let r = Roman::from_str("DXXIX");
        assert!(r.is_ok());
        let r = r.unwrap();
        assert_eq!(&vec![D, X, X, I, X], r.numerals());
        assert_eq!(529, *r);
    }

    #[test]
    fn from_str_invalid() {
        match Roman::from_str("DXSIX") {
            Err(Error::InvalidDigit(digit)) => assert_eq!('S', digit),
            _ => assert!(false)
        }
    }

    #[test]
    fn from_int_too_high() {
        match Roman::from(5003u32) {
            Err(Error::OutOfRange(digit)) => assert_eq!(5003, digit),
            _ => assert!(false)
        }
    }

    #[test]
    fn display_roman() {
        let r = Roman::from_str("DXXIX");
        assert!(r.is_ok());
        let r = r.unwrap();
        assert_eq!("DXXIX", r.to_string());
        assert_eq!("dxxix", r.to_lowercase());
        assert_eq!("DXXIX", format!("{}", r));
    }

    #[test]
    fn iteratate_numerals() {
        use self::Numeral::*;

        let r = Roman::from_str("VII");
        assert!(r.is_ok());
        let r = r.unwrap();
        let mut iter = r.iter();
        assert_eq!(Some(&V), iter.next());
        assert_eq!(Some(&I), iter.next());
        assert_eq!(Some(&I), iter.next());
        assert_eq!(None, iter.next());
    }


    #[test]
    fn creating_unchecked_zero() {
        let r = Roman::from_unchecked(0u32);
        assert_eq!(0, *r);
        let mut iter = r.iter();
        assert_eq!(None, iter.next());
    }

}
