use anyhow::Result;
use csv::ReaderBuilder;

use crate::csv::models::AssignedTransaction;

pub fn parse_assigned_transactions_csv(path: &str) -> Result<Vec<AssignedTransaction>> {
    let mut reader = ReaderBuilder::new()
        .delimiter(b',')
        .has_headers(false)
        .from_path(path)?;
    let mut items = Vec::new();
    for result in reader.deserialize() {
        let record: AssignedTransaction = result?;
        items.push(record)
    }
    Ok(items)
}
