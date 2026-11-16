use serde_json::from_str;
use std::{fs::File, io::Read, path::PathBuf};

#[allow(dead_code)]
fn get_strings_from_file(path: PathBuf) -> Result<Vec<String>, crate::DeserializationError> {
    let mut file = File::open(path)?;
    let mut file_content = String::new();
    let result: Result<usize, std::io::Error> = file.read_to_string(&mut file_content);
    match result {
        Ok(_) => deserialize_json(&file_content),
        Err(error) => return Err(error.into()),
    }
}

pub fn deserialize_json(raw: &str) -> Result<Vec<String>, crate::DeserializationError> {
    match from_str::<Vec<String>>(raw) {
        Ok(vector) => Ok(vector),
        Err(error) => Err(error.into()),
    }
}
