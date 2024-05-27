use serde::Deserialize;

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[allow(clippy::upper_case_acronyms)]
pub enum BudgetItemType {
    SING,
    MULTI,
    INFO,
}
