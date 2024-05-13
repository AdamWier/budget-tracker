mod csv_parser;

fn main() {
    let records = csv_parser::parse_transaction_csv("test.csv");
}
