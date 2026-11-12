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
}
