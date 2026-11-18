# anyword [![Crates.io](https://img.shields.io/crates/v/anyword)](https://crates.io/crates/anyword) [![Docs](https://img.shields.io/badge/docs-anyword-1370D3?style=flat-square&logo=rust)](https://docs.rs/anyword)

A crate that introduces the `WordsDictionary` data structure, which groups a series of words based on their length and provides functions to randomly retrieve them.

## Installation

### crates.io

Simply run the following command from the directory of your package.
```sh
cargo add anyword
```

### Cargo.toml

Add the following line to your `dependencies` section in the `Cargo.toml` file of your package:
```toml
anyword = "version" # (e.g., 0.1.0)
```

## Quick Start

```rust
fn random_food() -> Result<(), anyword::Error> {
    let raw_words = vec![
        "spaghetti",
        "hotdog",
        "donuts",
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
```

The previous code creates a vector of food words and creates a `WordsDictionary` with it. Then, using a `for` loop it gets 3 random words from the supplied list and prints them, propagating errors with the `?` operator if anything goes wrong at any step.

## Default Lists

Optionally, you may enable certain features such as `english` when adding this crate to your package in order to have access to `dictionaries` with a 2048-long list of words.

```rust
fn english_dict() -> Result<(), anyword::Error> {
    let english_dict = anyword::get_english_dict()?;
    for _ in 0..100 {
        println!("{}", english_dict.get_random()?);
    }
    Ok(())
}
```
