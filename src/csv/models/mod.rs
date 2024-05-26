mod budget_item;
mod deserializers;
pub mod list_item;
mod transaction;

pub use budget_item::BudgetItem;
pub use transaction::Transaction;

#[derive(Debug, Default, Clone)]
pub struct ParseResult {
    pub transactions: Vec<Transaction>,
    pub balance: f32,
}
