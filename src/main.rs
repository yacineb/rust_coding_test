use crate::cli::get_file_path;
use crate::datasource::csv_file_transactions_iterator;
use crate::transactions::*;
use std::error::Error;

mod cli;
mod transactions;

fn run() -> Result<(), Box<dyn Error>> {
    let file_path = get_file_path()?;
    let transactions = csv_file_transactions_iterator(file_path)?;
    let states = computation::compute(transactions);
    for state in states {
        println!("state {:?}", state);
    }
    Ok(())
}

fn main() {
    run().unwrap();
}
