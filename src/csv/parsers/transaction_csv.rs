use anyhow::{anyhow, Context, Result};
use csv::ReaderBuilder;
use encoding::all::ISO_8859_15;
use encoding::Encoding;
use std::fs::File;
use std::io::Read;

use crate::csv::models;
use crate::utils;

pub fn parse_transaction_csv(path: &str) -> Result<models::ParseResult> {
    let mut file_content = Vec::new();
    let mut file = File::open(path)?;
    file.read_to_end(&mut file_content)?;
    let encoded_file = ISO_8859_15
        .decode(&file_content, encoding::DecoderTrap::Replace)
        .map_err(|_| anyhow!("Could not get file encoding for {}", path))?;
    let parts: Vec<&str> = encoded_file.split("\r\n\r\n").collect();

    let transactions = get_transactions(
        parts
            .get(1)
            .with_context(|| format!("Could not get transactions parts for {}", path))?,
    );
    let balance = get_balance(
        parts
            .first()
            .with_context(|| format!("Could not get balance for {}", path))?,
    );
    Ok(models::ParseResult {
        balance,
        transactions,
    })
}

fn get_transactions(information: &str) -> Vec<models::Transaction> {
    let mut reader = ReaderBuilder::new()
        .delimiter(b';')
        .from_reader(information.as_bytes());
    let mut transactions = Vec::new();
    for result in reader.deserialize() {
        let record: models::Transaction = result.unwrap();
        transactions.push(record)
    }
    transactions
}

fn get_balance(information: &str) -> f32 {
    let mut reader = ReaderBuilder::new()
        .delimiter(b';')
        .from_reader(information.as_bytes());
    let mut balance = 0.0;
    for result in reader.records() {
        let line = result.expect("Problem in line with first half of CSV file");
        let label = line.get(0).expect("Cannot get label");
        if label.to_string().contains("Solde") {
            balance = utils::parse_european_number_format(line.get(1).expect("Cannot get balance"))
                .expect("Problem parsing balance");
            break;
        }
    }
    balance
}
