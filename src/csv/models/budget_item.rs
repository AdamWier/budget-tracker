use ratatui::text::Text;
use serde::Deserialize;

use super::list_item::ListItem;

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
