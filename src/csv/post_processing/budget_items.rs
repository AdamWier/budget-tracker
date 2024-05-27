use crate::csv::models::{BudgetItem, BudgetItemType};

pub fn add_spending_money(items: &mut Vec<BudgetItem>) {
    let total_expenses = items.iter().fold(0.0, |accu, item| {
        accu + (if item.code == "SAL" { 0.0 } else { item.amount })
    });
    let salary = items
        .iter()
        .position(|x| x.code == "SAL")
        .and_then(|x| items.get(x))
        .and_then(|x| Some(x.amount))
        .unwrap_or(0.0);
    let spending_money = salary - total_expenses;
    items.push(BudgetItem {
        label: "Spending money".to_string(),
        amount: spending_money,
        code: "SPEN".to_string(),
        setting: BudgetItemType::MULTI,
    });
}
