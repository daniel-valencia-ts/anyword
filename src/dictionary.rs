use crate::DictionaryError;
use crate::rng;
use std::collections::HashMap;
use std::collections::HashSet;

type Result<T> = std::result::Result<T, DictionaryError>;

/// A struct that stores words grouped by their length.
#[derive(Debug, PartialEq)]
pub struct WordsDictionary {
    words: HashMap<usize, HashSet<String>>,
}

impl WordsDictionary {
    /// Create a new `dictionary` with the supplied list of words.
    ///
    /// # Example
    ///
    /// ```
    /// use anyword::WordsDictionary;
    ///
    /// fn main() -> Result<(), anyword::DictionaryError> {
    ///     let words: Vec<String> = vec![
    ///         "sofa",
    ///         "chair",
    ///         "cushion",
    ///         "table"
    ///     ]
    ///     .into_iter()
    ///     .map(|str| str.to_string())
    ///     .collect();
    ///     let dict: WordsDictionary = WordsDictionary::new(words)?;
    ///
    ///     println!("{}", dict.get_random()?);
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Errors
    ///
    /// In order to create a new dictionary, a vector of [`String`]s with at least one element must
    /// be supplied. Otherwise, an [`EmptyVec`](DictionaryError::EmptyVec) error will be yielded.
    pub fn new(raw_words: Vec<String>) -> Result<Self> {
        if raw_words.len() == 0 {
            return Err(DictionaryError::EmptyVec);
        }
        let mut words: HashMap<usize, HashSet<String>> = HashMap::new();
        for word in raw_words {
            words
                .entry(word.len())
                .or_insert(HashSet::<String>::new())
                .insert(word);
        }
        Ok(WordsDictionary { words: words })
    }

    /// Get any word from the `dictionary` with a randomly generated
    /// length and index.
    ///
    /// # Errors
    ///
    /// If there are no values to choose from, an [`EmptyDictionary`](DictionaryError::EmptyDictionary)
    /// error will be returned.
    pub fn get_random(&self) -> Result<String> {
        let lengths: Vec<usize> = self.words.keys().copied().collect();
        if let Some(rand_len) = rng::from_values(&lengths) {
            self.get_random_with_len(rand_len)
        } else {
            Err(DictionaryError::EmptyDictionary)
        }
    }

    /// Get any word from the `dictionary` that matches the provided length
    /// with a randomly generated index.
    ///
    /// # Example
    ///
    /// ```
    /// use anyword::WordsDictionary;
    ///
    /// fn main() -> Result<(), anyword::DictionaryError> {
    ///     let words: Vec<String> = vec![
    ///         "fork",
    ///         "knife",
    ///         "spoon",
    ///     ]
    ///     .into_iter()
    ///     .map(|str| str.to_string())
    ///     .collect();
    ///     let dict: WordsDictionary = WordsDictionary::new(words)?;
    ///
    ///     assert_eq!(dict.get_random_with_len(4)?, "fork".to_string());
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Errors
    ///
    /// If there are no words that match the given length, an [`InvalidLength`](DictionaryError::InvalidLength)
    /// error will be returned.
    pub fn get_random_with_len(&self, word_len: usize) -> Result<String> {
        match self.words.get(&word_len) {
            Some(set) => {
                let set_len = set.len();
                match set_len {
                    0 => Err(DictionaryError::InvalidLength(word_len)),
                    _ => Ok(rng::from_values(&set.iter().collect()).unwrap().clone()),
                }
            }
            None => Err(DictionaryError::InvalidLength(word_len)),
        }
    }

    /// Insert a new word to a `dictionary`, returning true if the word was added,
    /// false if it was already present.
    ///
    /// Example
    ///
    /// ```
    /// # fn get_user_input() -> String {
    /// #    String::from("wubba lubba dub-dub!")
    /// # }
    /// use anyword::WordsDictionary;
    ///
    /// fn run() -> Result<(), anyword::DictionaryError> {
    ///     let mut dict: WordsDictionary = WordsDictionary::new(vec!["default".into()])?;
    ///     for _ in 0..10 {
    ///         let user_input: String = get_user_input();
    ///         dict.add(user_input);
    ///         println!("{}", dict.get_random()?);
    ///     }
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn add(&mut self, new_word: String) -> bool {
        self.words
            .entry(new_word.len())
            .or_insert(HashSet::<String>::new())
            .insert(new_word)
    }

    /// Remove a word from a `dictionary`, returning false if the word wasn't present,
    /// true otherwise.
    ///
    /// Example
    ///
    /// ```
    /// use anyword::WordsDictionary;
    ///
    /// fn main() -> Result<(), anyword::DictionaryError> {
    ///     let words: Vec<String> = vec![
    ///         "fork",
    ///         "knife",
    ///         "spoon",
    ///     ]
    ///     .into_iter()
    ///     .map(|str| str.to_string())
    ///     .collect();
    ///     let mut dict: WordsDictionary = WordsDictionary::new(words)?;
    ///
    ///     assert_eq!(dict.get_random_with_len(4)?, "fork".to_string());
    ///
    ///     dict.remove("fork");
    ///     assert!(dict.get_random_with_len(4).is_err());
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn remove(&mut self, target_word: &str) -> bool {
        match self.words.get_mut(&target_word.len()) {
            Some(entry) => entry.remove(target_word),
            None => false,
        }
    }

    /// Return whether the target word is present in the `dictionary` or not.
    pub fn contains(&self, target_word: String) -> bool {
        match self.words.get(&target_word.len()) {
            Some(entry) => entry.contains(&target_word),
            None => false,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn create_new() {
        let raw_words = vec![String::from("apple")];
        let dict = WordsDictionary::new(raw_words).unwrap();

        assert_eq!(dict.get_random(), Ok(String::from("apple")));
    }

    #[test]
    fn create_empty() {
        let empty_dict = WordsDictionary::new(Vec::<String>::new());
        assert_eq!(empty_dict, Err(DictionaryError::EmptyVec));
    }

    #[test]
    fn random_without_length() {
        let raw_words = vec![
            String::from("durian"),
            String::from("radical"),
            String::from("skyfire"),
            String::from("apolo"),
            String::from("flipside"),
        ];
        let dict = WordsDictionary::new(raw_words).unwrap();

        assert!(dict.get_random().is_ok())
    }

    #[test]
    fn random_with_length() {
        let raw_words = vec![
            String::from("realities"),
            String::from("bounce"),
            String::from("ikigai"),
            String::from("map"),
        ];
        let dict = WordsDictionary::new(raw_words).unwrap();

        assert_eq!(dict.get_random_with_len(3).unwrap(), String::from("map"));
    }
}
