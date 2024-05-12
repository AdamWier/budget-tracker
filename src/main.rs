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
    println!("{:?}", encoded_file);
    
    // let input = fs::read_to_string("test.csv").expect("No file found");
    // let mut parts = input.split("Date;Libellé;Montant(EUROS)");
    // parts.for_each(|x| println!("{:?}", x));
    // let mut part_1_file = File::create("part1.csv").expect("File cannot be created");
    // let mut reader = ReaderBuilder::new().delimiter(b';').from_path("test.csv").expect("No file found");
    // reader.records().for_each(|x| println!("{:?}", x));
}
