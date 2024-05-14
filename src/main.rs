mod csv;
mod utils;

use csv::parsers::{transaction_csv, budget_csv};

fn main() {
    let records = transaction_csv::parse_transaction_csv("test.csv");
    println!("{:#?}", records);

    let items = budget_csv::parse_budget_csv("budget.csv");
    println!("{:#?}", items);
}
