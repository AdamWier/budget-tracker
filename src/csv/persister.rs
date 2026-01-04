use std::{
    fs::OpenOptions,
    path::PathBuf,
    sync::{Arc, Mutex},
};

use anyhow::{anyhow, Context, Result};
use csv::Writer;

use crate::csv::{
    models::{AssignedTransaction, BudgetItem, CycleFile, Transaction},
    parsers::assigned_transactions::parse_assigned_transactions_csv,
};

pub fn persist_association(
    budget_item: BudgetItem,
    transaction: Transaction,
    assigned_transactions: &Arc<Mutex<Vec<AssignedTransaction>>>,
    current_file: &Arc<Mutex<CycleFile>>,
) -> Result<()> {
    let record = create_record(&transaction, &budget_item);
    let current_file = current_file
        .lock()
        .map_err(|_| anyhow!("Could not lock current file"))?;
    append_to_file(&current_file.path, &record)?;
    let mut assigned_transactions = assigned_transactions
        .lock()
        .map_err(|_| anyhow!("Could not lock assigned transactions"))?;
    assigned_transactions.clear();
    parse_assigned_transactions_csv(&current_file.path)?
        .into_iter()
        .for_each(|x| assigned_transactions.push(x));
    Ok(())
}

fn create_record(transaction: &Transaction, budget_item: &BudgetItem) -> Vec<String> {
    [
        String::from(budget_item.code.clone()),
        transaction.date.to_string(),
        transaction.label.to_string(),
        transaction.amount.to_string(),
    ]
    .to_vec()
}

fn append_to_file(path: &PathBuf, record: &Vec<String>) -> Result<()> {
    let file = OpenOptions::new().create(true).append(true).open(path)?;

    let mut writer = Writer::from_writer(file);
    writer
        .write_record(record)
        .with_context(|| format!("Problem writing record {:#?}", record))?;
    writer
        .flush()
        .with_context(|| format!("Problem flushing file, {:#?}", path))
}
