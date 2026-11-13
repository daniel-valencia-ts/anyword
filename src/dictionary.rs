use crate::rng;
use std::collections::HashMap;

/// A collection of words separated based on their length.
#[derive(Debug)]
pub struct WordsDictionary {
    pub words: HashMap<usize, Vec<String>>,
}

impl WordsDictionary {
    pub fn new(raw_words: Vec<String>) -> Self {
        let mut words: HashMap<usize, Vec<String>> = HashMap::new();
        for word in raw_words {
            match words.get_mut(&word.len()) {
                Some(vec) => vec.push(word),
                None => {
                    let length = word.len();
                    let vec = vec![word];
                    words.insert(length, vec);
                }
            }
        }
        WordsDictionary { words: words }
    }

    pub fn get(&self, length: usize, index: usize) -> Option<String> {
        match self.words.get(&length) {
            Some(vec) => match vec.get(index) {
                Some(word) => Some(word.clone()),
                None => None,
            },
            None => None,
        }
    }

    /// Get any word from the dictionary with a random length and index.
    pub fn get_random(&self) -> Option<String> {
        let lengths: Vec<usize> = self.words.keys().copied().collect();
        if let Some(rand_len) = rng::from_values(&lengths) {
            self.get(
                rand_len,
                rng::in_range(self.words.get(&rand_len).unwrap().len()),
            )
        } else {
            None
        }
    }

    /// Get a word from the dictionary that matches the given length.
    pub fn get_random_with_len(&self, len: usize) -> Option<String> {
        self.get(len, rng::in_range(self.words.get(&len).unwrap().len()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_new() {
        let raw_words = vec![String::from("apple"), String::from("basketball")];
        let dict = WordsDictionary::new(raw_words);

        assert_eq!(dict.get(5, 0), Some(String::from("apple")));
        assert_eq!(dict.get(usize::MAX, usize::MAX), None);
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
        let dict = WordsDictionary::new(raw_words);

        assert!(dict.get_random().is_some())
    }

    #[test]
    fn random_with_length() {
        let raw_words = vec![
            String::from("realities"),
            String::from("bounce"),
            String::from("ikigai"),
            String::from("map"),
        ];
        let dict = WordsDictionary::new(raw_words);

        assert_eq!(dict.get_random_with_len(3).unwrap(), String::from("map"));
    }
}
