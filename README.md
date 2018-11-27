# Septem

A library for parsing and working with Roman numerals.

Supports easy conversion from strings or numbers to roman numerals, and easy conversion back again.

**Written using the 2018 edition of Rust, so requires _beta_ or _nightly_.**


## Usage

```
use septem::{Roman};

let sept: Roman = "vii".parse().unwrap();
assert_eq!(7, *sept);
assert_eq!("VII", sept.to_string());
assert_eq!("vii", sept.to_lowercase());
```

The `use septem::prelude::*` is required to support the `std::str::{FromStr}` trait and the `Roman::from_str` function.
```
use septem::prelude::*;
use septem::{Roman};
let roman = Roman::from_str("dxxxii").unwrap();
assert_eq!(532, *roman);
```

### To Roman Numerals

Parsing from string is provided using Rust's `FromStr` trait.

```
let num: Roman = "XLII".parse().unwrap();
```

Parsing from an integer is done using `Roman::from`.

```
let num: Roman = Roman::from(42).unwrap();
```

### From Roman Numerals

A string representation of the roman numeral can be gotten using Rust's `Display` trait.

```
println!("Roman number: {}", Roman::from(42).unwrap());
```
There are also functions to get the string without going through the formatter; `to_string`, `to_lowercase` and `to_uppercase`. 

```
let dis = Roman::from(42).unwrap().to_string();
```

The numerical value of the roman numeral is available through Rust's `Deref` trait.

```
let roman = Roman::from(42).unwrap();
assert_eq!(42, *roman);
```

### Digits

If you need to work with the digits that make up the Roman numeral you can get those with the `to_digits` function.
```
let roman = Roman::from(42).unwrap();
roman.to_digits().iter().for_each(|i| {
  println!("digit: {}", i);
});
```

## Performance

Benchmarks for converting from a Roman numeral in string form to an integer, and the other way around are supplied. Testing against a few other Roman numeral libraries shows that this crate is performing on the same levels, or slightly faster than the alternatives. It is after all very important to have fast roman numeral conversion, can't have such an important part of a program be slow!


### Errors

Septem functions can return three kinds of errors
 - `InvalidDigit(char)`, when a char could not be parsed as a roman numeral
 - `InvalidNumber(u64)`, when a number could not be parsed as a single roman numeral
 - `OutOfRange(u32)`, when trying to convert a number less than, or equal to, `0` or larger than `3999`
