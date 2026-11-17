use std::{error, fmt};

/// Represents all errors that might take place when dealing with [dictionaries](crate::WordsDictionary).
#[derive(Debug, PartialEq)]
pub enum DictionaryError {
    /// Happens when an operation that requires at least one element doesn't have access to one.
    EmptyVec,
    /// Takes place when using a length that has no matching words.
    InvalidLength(usize),
    /// Occurs when attempting to access an index that is out of bounds in one of the
    /// dictionary's underlying vectors.
    InvalidIndex(usize),
}

/// A type that represents every error that could occur when using this crate.
#[derive(Debug)]
pub enum Error {
    /// Takes place when an operation related to dictionaries goes wrong.
    DictionaryError(DictionaryError),
    /// Occurs when an operation concerning deserialization fails.
    DeserializationError(serde_json::error::Error),
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

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::DictionaryError(..) => {
                write!(f, "an error with dictionaries took place")
            }
            Error::DeserializationError(..) => {
                write!(f, "a deserialization error took place")
            }
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Error::DictionaryError(e) => Some(e),
            Error::DeserializationError(e) => Some(e),
        }
    }
}

impl From<DictionaryError> for Error {
    fn from(err: DictionaryError) -> Error {
        Error::DictionaryError(err)
    }
}

impl From<serde_json::error::Error> for Error {
    fn from(err: serde_json::error::Error) -> Error {
        Error::DeserializationError(err)
    }
}
