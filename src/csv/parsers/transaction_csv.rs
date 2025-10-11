use anyhow::{anyhow, Result};
use csv::ReaderBuilder;
use encoding::all::ISO_8859_15;
use encoding::Encoding;
use std::fs::File;
use std::io::Read;

use crate::csv::models;

pub fn parse_transaction_csv(path: &str) -> Result<models::ParseResult> {
    let mut file_content = Vec::new();
    let mut file = File::open(path)?;
    file.read_to_end(&mut file_content)?;
    let encoded_file = ISO_8859_15
        .decode(&file_content, encoding::DecoderTrap::Replace)
        .map_err(|_| anyhow!("Could not get file encoding for {}", path))?;

    let transactions = get_transactions(&encoded_file)?;
    let balance = transactions.last().map(|x| x.balance).unwrap_or(0.0);
    Ok(models::ParseResult {
        balance,
        transactions,
    })
}

fn get_transactions(information: &str) -> Result<Vec<models::Transaction>> {
    let mut reader = ReaderBuilder::new()
        .delimiter(b';')
        .from_reader(information.as_bytes());
    let mut transactions = Vec::new();
    for result in reader.deserialize() {
        let record: models::Transaction = result?;
        transactions.push(record)
    }
    Ok(transactions)
}
