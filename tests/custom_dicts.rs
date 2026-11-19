use anyword::WordsDictionary;

type Result<T> = std::result::Result<T, anyword::Error>;

#[test]
fn random_food() -> Result<()> {
    let raw_words = vec![
        "spaghetti",
        "udon",
        "hotdog",
        "ramen",
        "steak",
        "donuts",
        "fries",
        "croissant",
        "cake",
    ]
    .into_iter()
    .map(|f| f.to_string())
    .collect();
    let food_dict = WordsDictionary::new(raw_words)?;

    for _ in 0..3 {
        println!("{}", food_dict.get_random()?);
    }
    Ok(())
}

#[test]
fn random_sport() -> Result<()> {
    let raw_words = vec![
        "basketball",
        "football",
        "skating",
        "volleyball",
        "baseball",
    ]
    .into_iter()
    .map(|f| f.to_string())
    .collect();
    let food_dict = WordsDictionary::new(raw_words)?;

    println!("{}", food_dict.get_random()?);
    Ok(())
}

#[test]
fn random_with_length() -> Result<()> {
    let raw_words = vec!["cap", "shirt", "pants", "shoe"]
        .into_iter()
        .map(|f| f.to_string())
        .collect();
    let dict = WordsDictionary::new(raw_words)?;

    assert_eq!(String::from("shoe"), dict.get_random_with_len(4)?);
    assert!(dict.get_random_with_len(usize::MAX).is_err());
    Ok(())
}

#[test]
fn empty_vec() {
    let vec = Vec::<String>::new();
    assert_eq!(
        WordsDictionary::new(vec),
        Err(anyword::DictionaryError::EmptyVec)
    );
}

#[test]
fn removal_and_insertion() -> Result<()> {
    let mut dict = WordsDictionary::new(vec!["tomato".into()])?;
    dict.add("potato".into());
    dict.add("cheese".into());

    dict.remove("tomato");

    assert!(dict.contains("cheese"));
    assert!(!dict.contains("tomato"));
    Ok(())
}
