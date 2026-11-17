#![cfg(feature = "defaultdicts")]
use anyword::{Error, WordsDictionary, defaultdicts};

type Result<T> = std::result::Result<T, anyword::Error>;

#[cfg(feature = "english")]
#[test]
fn english_dict() -> Result<()> {
    let english_dict = defaultdicts::english::get_dict()?;
    for _ in 0..100 {
        assert!(english_dict.get_random().is_ok())
    }
    Ok(())
}

#[cfg(feature = "spanish")]
#[test]
fn spanish_dict() -> Result<()> {
    let spanish_dict = defaultdicts::spanish::get_dict()?;
    for _ in 0..100 {
        assert!(spanish_dict.get_random().is_ok())
    }
    Ok(())
}
