#![cfg(any(feature = "english", feature = "spanish"))]

type Result<T> = std::result::Result<T, anyword::Error>;

#[cfg(feature = "english")]
#[test]
fn english_dict() -> Result<()> {
    let english_dict = anyword::get_english_dict()?;
    for _ in 0..100 {
        assert!(english_dict.get_random().is_ok())
    }
    Ok(())
}

#[cfg(feature = "spanish")]
#[test]
fn spanish_dict() -> Result<()> {
    let spanish_dict = anyword::get_spanish_dict()?;
    for _ in 0..100 {
        assert!(spanish_dict.get_random().is_ok())
    }
    Ok(())
}
