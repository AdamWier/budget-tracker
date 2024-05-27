use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct AssignedTransaction {
    pub code: String,
    pub date: String,
    pub label: String,
    pub amount: f32,
}
