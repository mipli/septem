[![Build Status](https://travis-ci.com/mipli/septem.svg?branch=master)](https://travis-ci.com/mipli/septem)
[![Crate](https://img.shields.io/crates/v/septem.svg)](https://crates.io/crates/septem)
[![API](https://docs.rs/septem/badge.svg)](https://docs.rs/septem)

# Septem

A library for parsing and working with Roman numerals.
Supports easy conversion from strings or numbers to roman numerals, and easy conversion back again.
including **Unicode Number Forms** and optional **archaic Roman numerals**.

## Usage

### Basic Example

Parsing from string is provided using Rust's `FromStr` trait.

```rust
extern crate septem;
use septem::{Roman};

let sept: Roman = "vii".parse().unwrap();
assert_eq!(7, *sept);
assert_eq!("VII", sept.to_string());
assert_eq!("vii", sept.to_lowercase());

// unicode
let sept: Roman = "Ⅶ".parse().unwrap();
assert_eq!(7, *sept);

// combination with unicode 
let septendecim: Roman = "XⅦ".parse().unwrap();
assert_eq!(17, *septendecim);
```

The `use septem::prelude::*` is required to support the `std::str::{FromStr}` trait and the `Roman::from_str` function.
```rust
extern crate septem;
use septem::prelude::*;
use septem::{Roman};

let roman = Roman::from_str("dxxxii").unwrap();
assert_eq!(532, *roman);
```

### To Roman Numerals

Convert integers into Roman numerals using either the `From` or `from_int` method:

```rust
use septem::Roman;

let num: Roman = Roman::from(42).unwrap();
println!("{}", num); // "XLII"
```

Or directly with digits:

```rust
use septem::Digit;

let digits = Digit::from_int(1994u32).unwrap();
assert_eq!(
    digits,
    vec![
        Digit::M, Digit::C, Digit::M, // 1900
        Digit::X, Digit::C,           // 90
        Digit::I, Digit::V            // 4
    ]
);
```

---

### From Roman Numerals

A Roman numeral can be converted back to a number easily:

```rust
use septem::Roman;

let roman = Roman::from(42).unwrap();
assert_eq!(42, *roman);
```

String conversions are handled by `Display`, `to_string`, or `to_lowercase`:

```rust
let roman = Roman::from(42).unwrap();
println!("Roman: {}", roman);            // "XLII"
println!("Lowercase: {}", roman.to_lowercase()); // "xlii"
```

The numerical value of the roman numeral is available through Rust's `Deref` trait.

```rust
let roman = Roman::from(42).unwrap();
assert_eq!(42, *roman);
```

---

### Unicode Support

`septem` automatically supports **Unicode Number Forms** (U+2160–U+217F):

```rust
use septem::Digit;

assert_eq!(Digit::from_char('Ⅷ').unwrap(), vec![Digit::V, Digit::I, Digit::I, Digit::I]);
assert_eq!(Digit::from_char('ⅳ').unwrap(), vec![Digit::I, Digit::V]);
```

---

### Optional Archaic Numerals

Enable ancient forms by turning on the `archaic` feature:

```bash
cargo add septem --features archaic
```

These include large-value symbols:

| Character | Value   | Name               |
| --------- | ------- | ------------------ |
| ↀ         | 1000    | One Thousand (old) |
| ↁ         | 5000    | Five Thousand      |
| ↂ         | 10 000  | Ten Thousand       |
| ↇ         | 50 000  | Fifty Thousand     |
| ↈ         | 100 000 | Hundred Thousand   |

Example:

```rust
#[cfg(feature = "archaic")]
{
    use septem::Digit;

    assert_eq!(Digit::from_char('ↀ').unwrap(), vec![Digit::OneThousandOld]);
    assert_eq!(Digit::from_char('ↁ').unwrap(), vec![Digit::FiveThousand]);
    assert_eq!(Digit::from_char('ↂ').unwrap(), vec![Digit::TenThousand]);
    assert_eq!(Digit::from_char('ↇ').unwrap(), vec![Digit::FiftyThousand]);
    assert_eq!(Digit::from_char('ↈ').unwrap(), vec![Digit::HundredThousand]);
}
```

---

### Working with Digits

You can access the component digits of a Roman numeral:

```rust
let roman = Roman::from(42).unwrap();
for d in roman.to_digits() {
    println!("Digit: {}", d);
}

```
## Performance

Benchmarks for converting from a Roman numeral in string form to an integer, and the other way around are supplied. Testing against a few other Roman numeral libraries shows that this crate is performing on the same levels, or slightly faster than the alternatives. It is after all very important to have fast roman numeral conversion, can't have such an important part of a program be slow!

The benchmarks cannot be run on stable Rust at the moment, so they should be run with the following command:
```
$ cargo +nightly bench
```


### Errors

Septem functions can return three kinds of errors
 - `InvalidDigit(char)`, when a char could not be parsed as a roman numeral
 - `InvalidNumber(u64)`, when a number could not be parsed as a single roman numeral
 - `OutOfRange(u32)`, when trying to convert a number less than, or equal to, `0` or larger than `3999`
