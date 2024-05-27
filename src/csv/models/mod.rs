mod assigned_transaction;
mod budget_item;
mod budget_item_type;
mod comparable_transaction;
mod deserializers;
pub mod list_item;
mod transaction;

pub use assigned_transaction::AssignedTransaction;
pub use budget_item::BudgetItem;
pub use budget_item_type::BudgetItemType;
pub use comparable_transaction::ComparableTransaction;
pub use transaction::Transaction;

#[derive(Debug)]
pub struct ParseResult {
    pub transactions: Vec<Transaction>,
    pub balance: f32,
}
