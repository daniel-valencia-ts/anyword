//! Get random words from the built-in lists or custom-made one
//!
//! # Quick Start
//!
//! This crate introduces the [WordsDictionary] struct, which stores a collection of words separated by length.
//! The [get_random](WordsDictionary::get_random) method is used to get a random word, whereas
//! [get_random_with_len](WordsDictionary::get_random_with_len) can be employed to
//! only get strings that match the specified length.
//!
//! ```
//! use anyword::WordsDictionary;
//!
//! let raw_words: Vec<String> = vec![String::from("crab"), String::from("penguin"), String::from("bear")];
//! let dict: WordsDictionary = WordsDictionary::new(raw_words).unwrap();
//!
//! let random_word: String = dict.get_random().unwrap();
//! println!("{random_word}"); // Could be "crab," "penguin," or "bear."
//! ```
//!
//! # Built-in Dictionaries
//!
//! Optionally, [features](https://doc.rust-lang.org/cargo/reference/features.html) such as `english`
//! or `spanish` might be activated to have access to dictionaries with a set of words in those languages.
//!
//! ```
//! use anyword::defaultdicts;
//! use anyword::WordsDictionary;
//!
//! // The "english" feature must be enabled for this to work.
//! let english_dict: WordsDictionary = defaultdicts::english::get_dict().unwrap();
//!
//! let random_word: String = english_dict.get_random().unwrap();
//! println!("{random_word}");
//! ```

#![warn(missing_docs)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#[cfg(feature = "defaultdicts")]
pub mod defaultdicts;
mod dictionary;
mod error;
mod rng;
mod utils;

pub use dictionary::*;
pub use error::*;
