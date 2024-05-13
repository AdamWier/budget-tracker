mod csv_parser;
mod utils;

fn main() {
    let records = csv_parser::parse_transaction_csv("test.csv");
    println!("{:#?}", records);
}
