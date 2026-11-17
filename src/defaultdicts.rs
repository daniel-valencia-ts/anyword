//! Default implementations of [WordsDictionary] in several languages.
//!
//! In order to use them, activate the feature flag of the language you want to use in the **Cargo.toml`
//! file at the root of your package.
//!
//! # Example
//!
//! ```
//! use anyword::defaultdicts;
//! use anyword::WordsDictionary;
//!
//! let spanish_dict = defaultdicts::spanish::get_dict().unwrap();
//! for i in 0..64 {
//!     println!("word number {i}: {}", spanish_dict.get_random().unwrap());
//! }
//! ```
use crate::WordsDictionary;
use crate::utils::deserialize_json;
type Result<T> = std::result::Result<T, crate::Error>;

#[cfg(feature = "english")]
pub mod english {
    use super::*;

    /// Build a 2048-word WordsDictionary of English words.
    #[cfg(feature = "english")]
    #[cfg_attr(docsrs, doc(cfg(feature = "english")))]
    pub fn get_dict() -> Result<WordsDictionary> {
        let raw = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/english.json"));
        let words = deserialize_json(raw)?;

        Ok(WordsDictionary::new(words)?)
    }
}

#[cfg(feature = "spanish")]
pub mod spanish {
    use super::*;

    /// Build a 2048-word WordsDictionary of Spanish words.
    #[cfg(feature = "spanish")]
    #[cfg_attr(docsrs, doc(cfg(feature = "spanish")))]
    pub fn get_dict() -> Result<WordsDictionary> {
        let raw = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/spanish.json"));
        let words = deserialize_json(raw)?;

        Ok(WordsDictionary::new(words)?)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[cfg(feature = "english")]
    #[test]
    fn random_english_word() -> Result<()> {
        let english_dict = english::get_dict()?;
        assert!(english_dict.get_random().is_ok());
        Ok(())
    }

    #[cfg(feature = "spanish")]
    #[test]
    fn random_spanish_word() -> Result<()> {
        let spanish_dict = english::get_dict()?;
        assert!(spanish_dict.get_random().is_ok());
        Ok(())
    }
}
