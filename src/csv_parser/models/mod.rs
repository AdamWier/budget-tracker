mod deserializers;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Transaction {
    date: String,
    #[serde(rename = "Libellé")]
    label: String,
    #[serde(rename = "Montant(EUROS)", deserialize_with = "deserializers::deserialize_amount")]
    amount: f32,
}

#[derive(Debug)]
pub struct ParseResult {
    pub transactions: Vec<Transaction>,
    pub balance: f32,
}