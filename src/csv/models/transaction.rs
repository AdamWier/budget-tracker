use ratatui::text::Text;
use serde::Deserialize;

use super::comparable_transaction::ComparableTransaction;
use super::deserializers;
use super::list_item::ListItem;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Transaction {
    pub date: String,
    #[serde(rename = "Libellé")]
    pub label: String,
    #[serde(
        rename = "Montant(EUROS)",
        deserialize_with = "deserializers::deserialize_amount"
    )]
    pub amount: f32,
}

impl ListItem for Transaction {
    fn get_list_label(&self) -> ratatui::prelude::Text {
        Text::raw(format!("{} - {} - {}", self.date, self.label, self.amount))
    }
    fn get_savable_value(&self) -> Vec<String> {
        vec![
            self.date.to_string(),
            self.label.to_string(),
            self.amount.to_string(),
        ]
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
