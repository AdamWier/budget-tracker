use serde::Deserialize;

use super::comparable_transaction::ComparableTransaction;

#[derive(Debug, Deserialize)]
pub struct AssignedTransaction {
    pub code: String,
    pub date: String,
    pub label: String,
    pub amount: f32,
}

impl ComparableTransaction for AssignedTransaction {
    fn get_comparable_value(&self) -> String {
        [
            self.date.to_string(),
            self.label.to_string(),
            self.amount.to_string(),
        ]
        .join("")
    }
}
