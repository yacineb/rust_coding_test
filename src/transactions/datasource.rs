use crate::dtos::Transaction;
use csv::ReaderBuilder;
use std::{fs::File, path::Path};

/// returns an iterator to the csv file content
pub fn csv_file_transactions_iterator(
    file_path: impl AsRef<Path>,
) -> Result<impl Iterator<Item = Transaction>, Box<dyn std::error::Error>> {
    let file = File::open(file_path)?;

    let iterator = ReaderBuilder::new()
        .has_headers(true)
        .trim(csv::Trim::All)
        .from_reader(file)
        .into_deserialize::<Transaction>()
        .map(|row| row.expect("Failed decoding row"));

    Ok(iterator)
}
