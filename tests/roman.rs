#[cfg(test)]
mod tests {
    use septem::prelude::*;
    use septem::{Roman, Error};

    #[test]
    fn from_valid() {
        let r = Roman::from(532u32);
        assert!(r.is_ok());
        let r = r.unwrap();
        assert_eq!(532, *r);
        assert_eq!("DXXXII", r.to_string());
    }

    #[test]
    fn from_unchecked() {
        let r = Roman::from_unchecked(5032u32);
        assert_eq!(5032, *r);
    }

    #[test]
    fn parse_str() {
        let r: Roman = "DXXIX".parse().unwrap();
        assert_eq!(529, *r);
    }

    #[test]
    fn from_str_valid() {
        let r = Roman::from_str("DXXIX");
        assert!(r.is_ok());
        let r = r.unwrap();
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
    fn to_numerals() {
        use septem::Numeral::*;
        let r = Roman::from(532u32);
        assert!(r.is_ok());
        let r = r.unwrap();
        assert_eq!(vec![D, X, X, X, I, I], r.to_numerals());
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
}
