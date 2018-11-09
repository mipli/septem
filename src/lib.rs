mod errors;
mod numeral;
mod roman;

pub mod prelude {
    pub use std::str::{FromStr};
}

pub use crate::{
    numeral::{Numeral},
    roman::{Roman},
    errors::{Result, Error}
};
