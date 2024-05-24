use std::io::Stdout;

use ratatui::{backend::CrosstermBackend, Terminal};

mod app;
mod components;
pub mod errors;
pub mod wrapper;

pub type Tui = Terminal<CrosstermBackend<Stdout>>;
pub use app::App;
