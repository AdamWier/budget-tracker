use csv::ReaderBuilder;
use encoding::all::ISO_8859_15;
use encoding::Encoding;
use std::fs::File;
use std::io::Read;

fn main() {
    let mut file_content = Vec::new();
    let mut file = File::open("test.csv").expect("Unable to open file");
    file.read_to_end(&mut file_content).expect("Unable to read");
    let encoded_file = ISO_8859_15.decode(&file_content, encoding::DecoderTrap::Replace).expect("Could not process file");
    let parts: Vec<&str> = encoded_file.split("\r\n\r\n").collect();
    
    let transactions = parts.get(1).expect("No transactions");
    let mut reader = ReaderBuilder::new().delimiter(b';').from_reader(transactions.as_bytes());
    reader.records().for_each(|x| println!("{:?}", x));
}
