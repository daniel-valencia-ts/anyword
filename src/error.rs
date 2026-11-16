use std::{error, fmt};

#[derive(Debug, PartialEq)]
pub enum DictionaryError {
    EmptyVec,
    InvalidLength(usize),
    InvalidIndex(usize),
}

#[derive(Debug)]
pub enum DeserializationError {
    IOError(std::io::Error),
    SerdeError(serde_json::error::Error),
}

#[derive(Debug)]
pub enum Error {
    DictionaryError(DictionaryError),
    DeserializationError(DeserializationError),
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

impl fmt::Display for DeserializationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DeserializationError::IOError(..) => {
                write!(f, "an error occurred while opening the file")
            }
            DeserializationError::SerdeError(..) => {
                write!(f, "couldn't deserialize the contents of the file")
            }
        }
    }
}

impl error::Error for DeserializationError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            DeserializationError::IOError(e) => Some(e),
            DeserializationError::SerdeError(e) => Some(e),
        }
    }
}

impl From<std::io::Error> for DeserializationError {
    fn from(err: std::io::Error) -> DeserializationError {
        DeserializationError::IOError(err)
    }
}

impl From<serde_json::error::Error> for DeserializationError {
    fn from(err: serde_json::error::Error) -> DeserializationError {
        DeserializationError::SerdeError(err)
    }
}

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

impl From<DeserializationError> for Error {
    fn from(err: DeserializationError) -> Error {
        Error::DeserializationError(err)
    }
}
