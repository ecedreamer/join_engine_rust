use csv::ReaderBuilder;
use serde_json::{json, Value};
use std::error::Error;
use std::fs::File;
use std::io::BufReader;

pub fn load_dataset(path: &str) -> Result<Vec<Value>, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut csv_reader = ReaderBuilder::new().has_headers(true).from_reader(reader);

    let headers = csv_reader
        .headers()?
        .iter()
        .map(|h| h.to_string())
        .collect::<Vec<_>>();

    let mut dataset = Vec::new();

    for result in csv_reader.records() {
        let record = result?;

        let mut json_obj = json!({});
        for (i, field) in record.iter().enumerate() {
            json_obj[&headers[i]] = Value::String(field.to_string());
        }

        dataset.push(json_obj);
    }

    Ok(dataset)
}
