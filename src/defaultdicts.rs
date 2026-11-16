use crate::dictionary::WordsDictionary;
use crate::utils::deserialize_json;
type Result<T> = std::result::Result<T, crate::utils::Error>;

#[cfg(feature = "english")]
mod english {
    use super::*;

    pub fn get_dict() -> Result<WordsDictionary> {
        let raw = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/english.json"));
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
}
