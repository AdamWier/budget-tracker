use serde::{Deserialize, Serialize};

use super::comparable_transaction::ComparableTransaction;

#[derive(Debug, Deserialize, Clone, PartialEq, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct AssignedTransaction {
    pub code: String,
    pub date: String,
    pub label: String,
    pub amount: f32,
}

impl ComparableTransaction for AssignedTransaction {
    fn get_comparable_value(&self) -> String {
        vec![
            self.date.to_string(),
            self.label.to_string(),
            self.amount.to_string(),
        ]
        .join("")
    }
}
