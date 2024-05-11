use csv::ReaderBuilder;

fn main() {
    let mut reader = ReaderBuilder::new().delimiter(b';').from_path("test.csv").expect("No file found");
    reader.records().for_each(|x| println!("{:?}", x));
}
