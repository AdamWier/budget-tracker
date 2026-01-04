use anyhow::{anyhow, Result};
use csv::ReaderBuilder;
use encoding::all::ISO_8859_15;
use encoding::Encoding;
use itertools::Itertools;
use std::fs::File;
use std::io::Read;

use crate::csv::models::{self, AssignedTransaction, ComparableTransaction};

pub fn parse_transaction_csv(
    path: &str,
    assigned_transactions: &Vec<AssignedTransaction>,
) -> Result<models::ParseResult> {
    let mut file_content = Vec::new();
    let mut file = File::open(path)?;
    file.read_to_end(&mut file_content)?;
    let encoded_file = ISO_8859_15
        .decode(&file_content, encoding::DecoderTrap::Replace)
        .map_err(|_| anyhow!("Could not get file encoding for {}", path))?;

    let transactions = get_transactions(&encoded_file)?;
    let balance = transactions.last().map(|x| x.balance).unwrap_or(0.0);

    let transactions_to_be_assigned = transactions
        .into_iter()
        .filter(|x| {
            !assigned_transactions
                .iter()
                .any(|y| x.get_comparable_value() == y.get_comparable_value())
        })
        .collect_vec();

    Ok(models::ParseResult {
        balance,
        transactions_to_be_assigned,
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
