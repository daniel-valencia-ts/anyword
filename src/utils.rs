use serde_json::from_str;

#[allow(dead_code)]
pub fn deserialize_json(raw: &str) -> Result<Vec<String>, crate::Error> {
    match from_str::<Vec<String>>(raw) {
        Ok(vector) => Ok(vector),
        Err(error) => Err(error.into()),
    }
}
