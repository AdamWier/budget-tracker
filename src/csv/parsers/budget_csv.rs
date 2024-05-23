use csv::ReaderBuilder;

use crate::csv::models::BudgetItem;

pub fn parse_budget_csv(path: &str) -> Vec<BudgetItem> {
    let mut reader = ReaderBuilder::new()
        .delimiter(b',')
        .has_headers(false)
        .from_path(path)
        .expect("Could not parse file");
    let mut items = Vec::new();
    for result in reader.deserialize() {
        let record: BudgetItem = result.unwrap();
        items.push(record)
    }
    items
}
