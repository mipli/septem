#[cfg(test)]
mod tests {
    extern crate septem;
    use self::septem::prelude::*;
    use self::septem::{Error, Roman};

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
            _ => assert!(false),
        }
    }

    #[test]
    fn from_int_too_high() {
        match Roman::from(5003u32) {
            Err(Error::OutOfRange(digit)) => assert_eq!(5003, digit),
            _ => assert!(false),
        }
    }

    #[test]
    fn to_digits() {
        use self::septem::Digit::*;
        let r = Roman::from(532u32);
        assert!(r.is_ok());
        let r = r.unwrap();
        assert_eq!(vec![D, X, X, X, I, I], r.to_digits());
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

    // --- Regression tests for from_str range + canonical-form enforcement ---
    //
    // Previously `from_str` bypassed the `Roman::from` range invariant
    // (1..=3999) and accepted non-canonical grammar, normalizing it on
    // round-trip (e.g. from_str("IIII").to_string() == "IV"). The fix makes
    // `from_str` enforce both invariants while keeping all canonical inputs
    // (including lowercase) working.

    #[test]
    fn from_str_rejects_range_bypass() {
        // from(4000)/from(8000)/from(0) are all Err(OutOfRange); from_str must
        // agree instead of building values the constructor forbids.
        match Roman::from_str("MMMM") {
            Err(Error::OutOfRange(v)) => assert_eq!(4000, v),
            other => panic!(
                "from_str(\"MMMM\") expected Err(OutOfRange), got {:?}",
                other
            ),
        }
        match Roman::from_str("MMMMMMMM") {
            Err(Error::OutOfRange(v)) => assert_eq!(8000, v),
            other => panic!(
                "from_str(\"MMMMMMMM\") expected Err(OutOfRange), got {:?}",
                other
            ),
        }
        match Roman::from_str("") {
            Err(Error::OutOfRange(v)) => assert_eq!(0, v),
            other => panic!("from_str(\"\") expected Err(OutOfRange), got {:?}", other),
        }
    }

    #[test]
    fn from_str_rejects_non_canonical_grammar() {
        // Each of these is invalid standard Roman grammar; the parser must
        // reject them with InvalidNumber (previously dead code) rather than
        // accept and normalize them.
        for s in &["IIII", "VV", "IC", "IIX", "XXXX", "MIM"] {
            match Roman::from_str(s) {
                Err(Error::InvalidNumber(_)) => {}
                other => panic!(
                    "from_str({:?}) expected Err(InvalidNumber), got {:?}",
                    s, other
                ),
            }
        }
    }

    #[test]
    fn from_str_accepts_canonical_controls() {
        // Valid canonical numerals still parse to the correct value.
        assert_eq!(1, *Roman::from_str("I").unwrap());
        assert_eq!(4, *Roman::from_str("IV").unwrap());
        assert_eq!(99, *Roman::from_str("XCIX").unwrap());
        assert_eq!(3999, *Roman::from_str("MMMCMXCIX").unwrap());
    }

    #[test]
    fn from_str_still_rejects_bad_char() {
        // Bad-char path is unchanged.
        match Roman::from_str("ABC") {
            Err(Error::InvalidDigit(digit)) => assert_eq!('A', digit),
            other => panic!(
                "from_str(\"ABC\") expected Err(InvalidDigit), got {:?}",
                other
            ),
        }
    }

    #[test]
    fn from_str_accepts_lowercase_canonical() {
        // `from_byte` accepts lowercase, so canonical lowercase inputs must
        // still parse (matches the lib.rs doctests: "vii", "dxxxii").
        assert_eq!(7, *Roman::from_str("vii").unwrap());
        assert_eq!(532, *Roman::from_str("dxxxii").unwrap());
        assert_eq!(3999, *Roman::from_str("mmmcmxcix").unwrap());
    }

    #[test]
    fn from_str_canonical_round_trip_property() {
        // For every value in range, from(n).to_string() must parse back to n
        // (canonical inputs are accepted); and the non-canonical spelling of
        // the same value must be rejected.
        for n in 1u32..=3999 {
            let canonical = Roman::from(n).unwrap().to_string();
            assert_eq!(n, *Roman::from_str(&canonical).unwrap());
        }
        // A representative set of non-canonical spellings must all be rejected.
        for s in &["IIII", "VV", "IC", "IIX", "XXXX", "MIM", "VIIII", "LXXXX"] {
            assert!(
                Roman::from_str(s).is_err(),
                "from_str({:?}) should be Err",
                s
            );
        }
    }
}
