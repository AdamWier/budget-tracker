use std::io::Stdout;

use ratatui::{backend::CrosstermBackend, Terminal};

pub mod app;
pub mod app_builder;
mod components;
pub mod state;
pub mod wrapper;

type Tui = Terminal<CrosstermBackend<Stdout>>;
