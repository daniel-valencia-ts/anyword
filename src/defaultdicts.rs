use crate::WordsDictionary;
use crate::utils::deserialize_json;
type Result<T> = std::result::Result<T, crate::Error>;

#[cfg(feature = "english")]
pub mod english {
    use super::*;

    pub fn get_dict() -> Result<WordsDictionary> {
        let raw = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/english.json"));
        let words = deserialize_json(raw)?;

        Ok(WordsDictionary::new(words)?)
    }
}

#[cfg(feature = "spanish")]
pub mod spanish {
    use super::*;

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
