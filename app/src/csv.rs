use serde::{Deserialize, Serialize};
use std::error::Error;

pub fn read_file<'a, T>(path: &str) -> Result<Vec<T>, Box<dyn Error>>
where
    T: std::fmt::Debug + Serialize + for<'de> Deserialize<'de>,
{
    let mut reader = csv::Reader::from_path(path)?;
    let mut data: Vec<T> = Vec::new();
    for result in reader.deserialize() {
        // Notice that we need to provide a type hint for automatic de-serialization.
        let record: T = result?;
        data.push(record)
    }
    Ok(data)
}

pub fn write_file<T>(path: &str, data: Vec<T>) -> Result<(), Box<dyn Error>>
where
    T: Serialize,
{
    let mut writer = csv::Writer::from_path(path)?;
    for elem in data {
        writer.serialize(elem)?;
    }
    Ok(())
}
