//! Get random words from the built-in lists or custom-made one
//!
//! # Quick Start
//!
//! This crate introduces the [`WordsDictionary`] struct (also referred simply as "`dictionary`"
//! throughout this documentation), which stores a collection of words separated by length.
//! The [`get_random`](WordsDictionary::get_random) method is used to get a random word, whereas
//! [`get_random_with_len`](WordsDictionary::get_random_with_len) can be employed to
//! only get strings that match the specified length.
//!
//! ```
//! use anyword::WordsDictionary;
//!
//! fn print_random_word() -> Result<(), anyword::Error> {
//!     let raw_words: Vec<String> = vec!["crab".into(), "penguin".into(), "bear".into()];
//!     let dict: WordsDictionary = WordsDictionary::new(raw_words);
//!
//!     let random_word: String = dict.get_random()?;
//!     println!("{random_word}"); // Could be "crab," "penguin," or "bear."
//!     Ok(())
//! }
//! ```
//!
//! # Built-in Dictionaries
//!
//! Optionally, [features](https://doc.rust-lang.org/cargo/reference/features.html) such as `english`
//! or `spanish` might be activated to have access to `dictionaries` with a set of words in those languages.
//!
//! ```
//! use anyword::WordsDictionary;
//!
//! fn print_english_words() -> Result<(), anyword::Error> {
//!     // The "english" feature must be enabled for this to work.
//!     let english_dict: WordsDictionary = anyword::get_english_dict()?;
//!
//!     for _ in 0..16 {
//!         println!("{}", english_dict.get_random()?)
//!     }
//!     Ok(())
//! }
//! ```

#![warn(missing_docs)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#[cfg(any(feature = "english", feature = "spanish", feature = "french"))]
mod defaultdicts;
mod dictionary;
mod error;
mod rng;
mod utils;

#[cfg(any(feature = "english", feature = "spanish", feature = "french"))]
pub use defaultdicts::*;
pub use dictionary::*;
pub use error::*;
