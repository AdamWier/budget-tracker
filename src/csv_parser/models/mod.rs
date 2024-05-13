pub mod deserializers;
use serde::Deserialize;
use deserializers::deserialize_amount;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Transaction {
    date: String,
    #[serde(rename = "Libellé")]
    label: String,
    #[serde(rename = "Montant(EUROS)", deserialize_with = "deserialize_amount")]
    amount: f32,
}