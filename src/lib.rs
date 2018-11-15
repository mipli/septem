//! # Septem
//!
//! A library for working with roman numerals. Converting at will between strings, integers and roman
//! numerals.
//!
//! # Examples
//! ```rust
//! use septem::prelude::*;
//! use septem::*;
//!
//! let sept: Roman = "vii".parse().unwrap();
//! assert_eq!(7, *sept);
//! assert_eq!("VII", sept.to_string());
//! assert_eq!("vii", sept.to_lowercase());
//! ```

mod errors;
mod digit;
mod roman;

pub mod prelude {
    pub use std::str::{FromStr};
}

pub use crate::{
    digit::{Digit},
    roman::{Roman},
    errors::{Result, Error}
};
