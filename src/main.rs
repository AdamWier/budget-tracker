mod csv;
mod ui;
mod utils;

use color_eyre::eyre::Result;
use csv::parsers::{budget_csv, transaction_csv};

use crate::ui::App;

fn main() -> Result<()> {
    let parse_result = transaction_csv::parse_transaction_csv("test.csv");
    let _items = budget_csv::parse_budget_csv("budget.csv");

    ui::errors::install_hooks()?;
    let mut terminal = ui::wrapper::init()?;
    App::new(parse_result).run(&mut terminal)?;
    ui::wrapper::restore()?;
    Ok(())
}
