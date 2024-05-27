use serde::Deserialize;

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub enum BudgetItemType {
    SING,
    MULTI,
    INFO,
}
