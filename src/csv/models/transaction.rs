use ratatui::text::Text;
use serde::Deserialize;

use super::comparable_transaction::ComparableTransaction;
use super::list_item::ListItem;

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Transaction {
    pub date: String,
    #[serde(rename = "Libellé")]
    pub label: String,
    #[serde(rename = "Montant")]
    pub amount: f32,
    #[serde(alias = "Solde")]
    pub balance: f32,
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
