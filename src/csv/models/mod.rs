mod deserializers;

use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Transaction {
    pub date: String,
    #[serde(rename = "Libellé")]
    pub label: String,
    #[serde(rename = "Montant(EUROS)", deserialize_with = "deserializers::deserialize_amount")]
    pub amount: f32,
}

#[derive(Debug, Default, Clone)]
pub struct ParseResult {
    pub transactions: Vec<Transaction>,
    pub balance: f32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BudgetItem {
    label: String,
    amount: f32,
}