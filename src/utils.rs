use serde_json::from_str;
use std::{error, fmt, fs::File, io::Read, path::PathBuf};

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

#[derive(Debug)]
pub enum DeserializationError {
    IOError(std::io::Error),
    SerdeError(serde_json::error::Error),
}

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

#[derive(Debug)]
pub enum Error {
    DictionaryError(DictionaryError),
    DeserializationError(DeserializationError),
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

pub fn get_strings_from_file(path: PathBuf) -> Result<Vec<String>, DeserializationError> {
    let mut file = File::open(path)?;
    let mut file_content = String::new();
    let result: Result<usize, std::io::Error> = file.read_to_string(&mut file_content);
    match result {
        Ok(_) => deserialize_json(&file_content),
        Err(error) => return Err(error.into()),
    }
}

pub fn deserialize_json(raw: &str) -> Result<Vec<String>, DeserializationError> {
    match from_str::<Vec<String>>(raw) {
        Ok(vector) => Ok(vector),
        Err(error) => Err(error.into()),
    }
}
