use crate::cli::get_file_path;
use crate::datasource::csv_file_transactions_iterator;
use crate::transactions::*;
use std::error::Error;

mod cli;
mod exporters;
mod transactions;

fn run() -> Result<(), Box<dyn Error>> {
    let file_path = get_file_path()?;
    let transactions = csv_file_transactions_iterator(file_path)?;
    let accounts = computation::compute(transactions);

    exporters::export_as_csv(std::io::stdout(), accounts)?;
    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Application error: {}", e);
        std::process::exit(1);
    }
}
