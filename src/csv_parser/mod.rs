mod models;

use csv::ReaderBuilder;
use encoding::all::ISO_8859_15;
use encoding::Encoding;
use std::fs::File;
use std::io::Read;

pub fn parse_transaction_csv(path: &str) -> Vec<models::Transaction> {
    let mut file_content = Vec::new();
    let mut file = File::open(path).expect("Unable to open file");
    file.read_to_end(&mut file_content).expect("Unable to read file");
    let encoded_file = ISO_8859_15.decode(&file_content, encoding::DecoderTrap::Replace).expect("Could not decode file");
    let parts: Vec<&str> = encoded_file.split("\r\n\r\n").collect();

    let transactions = parts.get(1).expect("No transactions found");
    let mut reader = ReaderBuilder::new().delimiter(b';').from_reader(transactions.as_bytes());
    let mut records = Vec::new();
    for result in reader.deserialize() {
        let record: models::Transaction = result.unwrap();
        println!("{:#?}", record);
        records.push(record)
    }
    records
}
