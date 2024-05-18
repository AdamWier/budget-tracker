mod csv;
mod utils;
mod ui;

use color_eyre::eyre::Result;
use csv::parsers::{transaction_csv, budget_csv};

fn main() -> Result<()>{
    let parse_result = transaction_csv::parse_transaction_csv("test.csv");
    println!("{:#?}", parse_result);

    let items = budget_csv::parse_budget_csv("budget.csv");
    println!("{:#?}", items);

    ui::errors::install_hooks()?;
    let mut terminal = ui::wrapper::init()?;
    ui::wrapper::restore()?;
    Ok(())
}
