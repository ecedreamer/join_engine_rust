use csv::Reader;
use serde::de::DeserializeOwned;

pub fn load_dataset<T: DeserializeOwned>(file_path: &str) -> Result<Vec<T>, String> {
    let mut reader = Reader::from_path(file_path).unwrap();
    let mut items: Vec<T> = Vec::new();
    for result in reader.deserialize() {
        let item: T = result.map_err(|e| format!("Error deserializing: {}", e))?;
        items.push(item);
    }
    Ok(items)
}
