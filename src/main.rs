use crate::cli::get_file_path;
use crate::payment::compute_account_statues;
use crate::transactions::*;
use std::error::Error;

mod cli;
mod transactions;

fn run() -> Result<(), Box<dyn Error>> {
    let file_path = get_file_path()?;
    let records = datasource::csv_file_transactions_iterator(file_path)?;
    let states = compute_account_statues(records);
    for state in states {
        println!("state {:?}", state);
    }
    Ok(())
}

fn main() {
    run().unwrap();
}
