use crate::csv::models::{AssignedTransaction, ComparableTransaction, Transaction};

pub fn remove_already_processed_items(
    transactions: &mut Vec<Transaction>,
    assigned_transactions: &[AssignedTransaction],
) {
    transactions.retain(|x| {
        !assigned_transactions
            .iter()
            .any(|y| x.get_comparable_value() == y.get_comparable_value())
    });
}
