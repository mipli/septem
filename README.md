# Septem

A library for parsing and working with Roman numerals.

Supports easy conversion from strings or numbers to roman numerals, and easy conversion back again.


## Usage

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


### Errors

Septem functions can return three kinds of errors
 - `InvalidDigit(char)`, when a char could not be parsed as a roman numeral
 - `InvalidNumber(u64)`, when a number could not be parsed as a single roman numeral
 - `OutOfRange(u32)`, when trying to convert a number less than, or equal to, `0` or larger than `3999`
