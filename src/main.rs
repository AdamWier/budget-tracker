mod csv_parser;

fn main() {
    let records = csv_parser::parse_csv("test.csv");
}
