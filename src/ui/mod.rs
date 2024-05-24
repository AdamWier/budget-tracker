mod components;
pub mod errors;
pub mod wrapper;

use crate::csv::models::{BudgetItem, ParseResult};
use color_eyre::eyre::{Context, Result};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::prelude::*;
use std::io::Stdout;

use self::components::{main_layout::MainLayout, Component};

pub type Tui = Terminal<CrosstermBackend<Stdout>>;

#[derive(Debug)]
pub struct App {
    exit: bool,
    main_layout: MainLayout,
}

impl App {
    pub fn new(parse_result: ParseResult, budget_items: Vec<BudgetItem>) -> App {
        App {
            main_layout: MainLayout::init(parse_result, budget_items),
            exit: false,
        }
    }
    pub fn run(&mut self, terminal: &mut Tui) -> Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.render_frame(frame))?;
            self.handle_events().wrap_err("handle events failed")?;
        }
        Ok(())
    }

    fn render_frame(&mut self, frame: &mut Frame) {
        self.main_layout.render(frame, frame.size());
    }

    fn handle_events(&mut self) -> Result<()> {
        let event = event::read()?;
        match event {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => self
                .handle_key_event(key_event)
                .wrap_err_with(|| format!("handling key event failed:\n{key_event:#?}")),
            _ => Ok(()),
        }?;
        self.main_layout.handle_events(&event)?;
        Ok(())
    }

    pub fn handle_key_event(&mut self, key_event: KeyEvent) -> Result<()> {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            _ => {}
        }
        Ok(())
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}
