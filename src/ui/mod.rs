use std::io::Stdout;

use ratatui::{backend::CrosstermBackend, Terminal};

pub mod app;
pub mod app_builder;
mod components;
pub mod errors;
pub mod wrapper;

type Tui = Terminal<CrosstermBackend<Stdout>>;
