mod deserializers;
pub mod list_item;

use ratatui::text::Text;
use serde::Deserialize;

use self::list_item::ListItem;

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
}

#[derive(Debug, Default, Clone)]
pub struct ParseResult {
    pub transactions: Vec<Transaction>,
    pub balance: f32,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct BudgetItem {
    pub label: String,
    pub amount: f32,
}

impl ListItem for BudgetItem {
    fn get_list_label(&self) -> ratatui::prelude::Text {
        Text::raw(format!("{} - {}", self.label, self.amount))
    }
}
