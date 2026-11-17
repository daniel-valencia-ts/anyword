use crate::WordsDictionary;
use crate::utils::deserialize_json;
type Result<T> = std::result::Result<T, crate::Error>;

/// Build a 2048-word WordsDictionary of English words.
///
/// # Example
///
/// ```
/// use anyword::WordsDictionary;
///
/// let english_dict: WordsDictionary = anyword::get_english_dict().unwrap();
/// for i in 0..64 {
///     println!("word number {i}: {}", english_dict.get_random().unwrap());
/// }
/// ```
#[cfg(feature = "english")]
#[cfg_attr(docsrs, doc(cfg(feature = "english")))]
pub fn get_english_dict() -> Result<WordsDictionary> {
    let raw = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/english.json"));
    let words = deserialize_json(raw)?;

    Ok(WordsDictionary::new(words)?)
}

/// Build a 2048-word WordsDictionary of Spanish words.
///
/// # Example
///
/// ```
/// use anyword::WordsDictionary;
///
/// let spanish_dict: WordsDictionary = anyword::get_spanish_dict().unwrap();
/// for i in 0..64 {
///     println!("word number {i}: {}", spanish_dict.get_random().unwrap());
/// }
/// ```
#[cfg(feature = "spanish")]
#[cfg_attr(docsrs, doc(cfg(feature = "spanish")))]
pub fn get_spanish_dict() -> Result<WordsDictionary> {
    let raw = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/spanish.json"));
    let words = deserialize_json(raw)?;

    Ok(WordsDictionary::new(words)?)
}

#[cfg(test)]
mod test {
    use super::*;

    #[cfg(feature = "english")]
    #[test]
    fn random_english_word() -> Result<()> {
        let english_dict = get_english_dict()?;
        assert!(english_dict.get_random().is_ok());
        Ok(())
    }

    #[cfg(feature = "spanish")]
    #[test]
    fn random_spanish_word() -> Result<()> {
        let spanish_dict = get_spanish_dict()?;
        assert!(spanish_dict.get_random().is_ok());
        Ok(())
    }
}
