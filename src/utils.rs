use std::{error, fmt};

#[derive(Debug, PartialEq)]
pub enum DictionaryError {
    EmptyVec,
    InvalidLength(usize),
    InvalidIndex(usize),
}

impl fmt::Display for DictionaryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DictionaryError::EmptyVec => write!(f, "the vector must have at least one element"),
            DictionaryError::InvalidLength(len) => {
                write!(f, "no strings match the given length: {len}")
            }
            DictionaryError::InvalidIndex(idx) => {
                write!(f, "no string at the target index: {idx}")
            }
        }
    }
}

impl error::Error for DictionaryError {}
