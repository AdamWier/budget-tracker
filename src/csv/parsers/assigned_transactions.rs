use std::path::PathBuf;

use anyhow::{Context, Result};
use csv::ReaderBuilder;
use itertools::Itertools;

use crate::csv::models::AssignedTransaction;

pub fn parse_assigned_transactions_csv(path: &PathBuf) -> Result<Vec<AssignedTransaction>> {
    ReaderBuilder::new()
        .delimiter(b',')
        .has_headers(false)
        .from_path(path)?
        .deserialize()
        .map(|x| x.context("Could not deserialize csv"))
        .try_collect()
}
