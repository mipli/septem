//! # Septem
//!
//! A library for working with roman numerals. Converting at will between strings, integers and roman
//! numerals.
//!
//! # Examples
//! ```rust
//! extern crate septem;
//! use septem::{Roman};
//!
//! let sept: Roman = "vii".parse().unwrap();
//! assert_eq!(7, *sept);
//! assert_eq!("VII", sept.to_string());
//! assert_eq!("vii", sept.to_lowercase());
//! ```
//!
//! The `prelude` include is required to support from `std::str::{FromStr}` trait and the
//! `Roman::from_str` function.
//!
//! ```rust
//! extern crate septem;
//! use septem::prelude::*;
//! use septem::{Roman};
//!
//! let roman = Roman::from_str("dxxxii").unwrap();
//! assert_eq!(532, *roman);
//! ```

mod digit;
mod errors;
mod roman;

pub mod prelude {
    pub use std::str::FromStr;
}

pub use crate::{
    digit::Digit,
    errors::{Error, Result},
    roman::Roman,
};
