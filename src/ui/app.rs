use color_eyre::eyre::{Context, Result};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use notify::{ReadDirectoryChangesWatcher, Watcher};
use ratatui::prelude::*;
use std::{io::Stdout, path::Path};

use super::components::{layouts::main_layout::MainLayout, Component};

pub type Tui = Terminal<CrosstermBackend<Stdout>>;

#[derive(Debug)]
pub struct App<'a> {
    pub exit: bool,
    pub main_layout: MainLayout<'a>,
    pub watcher: ReadDirectoryChangesWatcher,
}

#[allow(clippy::single_match)]
impl App<'_> {
    pub fn run(&mut self, terminal: &mut Tui) -> Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.render_frame(frame))?;
            self.handle_events().wrap_err("handle events failed")?;
        }
        self.watcher
            .unwatch(Path::new("assigned_transactions.csv"))?;
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
