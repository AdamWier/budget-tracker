use std::fs::OpenOptions;

use csv::Writer;

use crate::csv::models::list_item::ListItem;

pub fn persist_association<T: ListItem + ?Sized>(transaction: &T, budget_item: &T) {
    let record = create_record(transaction, budget_item);
    append_to_file("assigned_transactions.csv", &record)
}

fn create_record<T: ListItem + ?Sized>(transaction: &T, budget_item: &T) -> Vec<String> {
    let transaction_save_value = transaction.get_savable_value();
    let budget_save_value = budget_item.get_savable_value();
    [transaction_save_value, budget_save_value].concat()
}

fn append_to_file(path: &str, record: &Vec<String>) {
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .unwrap();

    let mut writer = Writer::from_writer(file);
    writer.write_record(record).expect("Problem writing record");
    writer.flush().expect("Problem flushing file")
}
