use std::{
    fs::OpenOptions,
    sync::{Arc, Mutex},
};

use anyhow::{Context, Result};
use csv::Writer;

use crate::{
    consts::ASSIGNED_TRANSACTIONS_FILE_NAME,
    csv::{
        models::{list_item::ListItem, AssignedTransaction},
        parsers::assigned_transactions::parse_assigned_transactions_csv,
    },
};

pub fn persist_association<T: ListItem + ?Sized>(
    transaction: &T,
    budget_item: &T,
    assigned_transactions: &Arc<Mutex<Vec<AssignedTransaction>>>,
) -> Result<()> {
    let record = create_record(transaction, budget_item);
    append_to_file(ASSIGNED_TRANSACTIONS_FILE_NAME, &record)?;
    let mut assigned_transactions = assigned_transactions.lock().unwrap();
    assigned_transactions.clear();
    parse_assigned_transactions_csv(ASSIGNED_TRANSACTIONS_FILE_NAME)?
        .into_iter()
        .for_each(|x| assigned_transactions.push(x));
    Ok(())
}

fn create_record<T: ListItem + ?Sized>(transaction: &T, budget_item: &T) -> Vec<String> {
    let transaction_save_value = transaction.get_savable_value();
    let budget_save_value = budget_item.get_savable_value();
    [transaction_save_value, budget_save_value].concat()
}

fn append_to_file(path: &str, record: &Vec<String>) -> Result<()> {
    let file = OpenOptions::new().create(true).append(true).open(path)?;

    let mut writer = Writer::from_writer(file);
    writer
        .write_record(record)
        .with_context(|| format!("Problem writing record {:#?}", record))?;
    writer
        .flush()
        .with_context(|| format!("Problem flushing file, {}", path))
}
