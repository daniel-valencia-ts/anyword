//! Randomly get words from the default list or an external file.
#[cfg(feature = "defaultdicts")]
pub mod defaultdicts;
pub mod dictionary;
mod error;
pub mod rng;
pub mod utils;

pub use error::*;
