use csv::ReaderBuilder;
use encoding::all::ISO_8859_15;
use encoding::Encoding;
use std::fs::File;
use std::io::Read;
use serde::{Deserialize, Deserializer};

fn deserialize_amount<'de, D>(deserializer: D) -> Result<f32, D::Error>
where
    D: Deserializer<'de>,
{
    let value: &str = Deserialize::deserialize(deserializer).expect("Cannot deserialize value");
    let error = format!("Error parsing amount {}", value);

    let parts: Vec<&str> = value.split(',').collect();
    let main_part = parts.first().expect(&error).parse::<f32>().expect(&error);
    let decimal_part = parts.get(1).expect(&error).parse::<f32>().expect(&error) / 100.0;
    Ok(main_part + decimal_part)
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Transaction {
    date: String,
    #[serde(rename = "Libellé")]
    label: String,
    #[serde(rename = "Montant(EUROS)", deserialize_with = "deserialize_amount")]
    amount: f32,
}



fn main() {
    let mut file_content = Vec::new();
    let mut file = File::open("test.csv").expect("Unable to open file");
    file.read_to_end(&mut file_content).expect("Unable to read file");
    let encoded_file = ISO_8859_15.decode(&file_content, encoding::DecoderTrap::Replace).expect("Could not decode file");
    let parts: Vec<&str> = encoded_file.split("\r\n\r\n").collect();

    let transactions = parts.get(1).expect("No transactions found");
    let mut reader = ReaderBuilder::new().delimiter(b';').from_reader(transactions.as_bytes());

    for result in reader.deserialize() {
        let record: Transaction = result.unwrap();
        println!("{:#?}", record);
    }
}
