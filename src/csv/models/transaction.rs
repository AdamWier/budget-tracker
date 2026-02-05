use std::str::FromStr;

use chrono::NaiveDate;
use itertools::Itertools;
use ratatui::text::Text;
use serde::de::Error;
use serde::{Deserialize, Deserializer};

use super::comparable_transaction::ComparableTransaction;
use super::list_item::ListItem;

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Transaction {
    #[serde(alias = "Date", deserialize_with = "deserialze_european_date")]
    pub date: NaiveDate,
    #[serde(rename = "Libellé")]
    pub label: String,
    #[serde(rename = "Montant")]
    pub amount: f32,
    #[serde(alias = "Solde")]
    pub balance: f32,
}

fn deserialze_european_date<'de, D>(d: D) -> Result<NaiveDate, D::Error>
where
    D: Deserializer<'de>,
{
    let string = String::deserialize(d)?;
    let (day, month, year) = string
        .split("/")
        .collect_tuple()
        .ok_or(Error::custom("Missing date parts"))?;
    let date_string = format!("{year}-{month}-{day}");
    NaiveDate::from_str(&date_string).map_err(Error::custom)
}

impl ListItem<Transaction> for Transaction {
    fn get_list_label(&self) -> ratatui::prelude::Text {
        Text::raw(format!("{} - {} - {}", self.date, self.label, self.amount))
    }
    fn get_savable_value(&self) -> Transaction {
        self.clone()
    }
}

impl ComparableTransaction for Transaction {
    fn get_comparable_value(&self) -> String {
        [
            self.date.to_string(),
            self.label.to_string(),
            self.amount.to_string(),
        ]
        .join("")
    }
}
