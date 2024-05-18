pub mod errors;
pub mod wrapper;
// mod components;

// use color_eyre::eyre::{Context, Result};
// use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::prelude::*;
use std::io::Stdout;
// use crate::csv::models::ParseResult;

pub type Tui = Terminal<CrosstermBackend<Stdout>>;

// #[derive(Debug, Default)]
// pub struct App {
//     exit: bool,
//     parse_result: ParseResult
// }

// impl App {
//     pub fn new(parse_result: ParseResult) -> App {
//         let mut app = App::default();
//         app.parse_result = parse_result;
//         app
//     }
//     pub fn run(&mut self, terminal: &mut Tui) -> Result<()> {
//         while !self.exit {
//             terminal.draw(|frame| self.render_frame(frame))?;
//             self.handle_events().wrap_err("handle events failed")?;
//         }
//         Ok(())
//     }

//     fn render_frame(&self, frame: &mut Frame) {
//         // let main_layout = MainLayout::init(frame, &self.parse_result);
//         // main_layout.render_widgets(frame);
        
//     }

//     fn handle_events(&mut self) -> Result<()> {
//         match event::read()? {
//             Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
//                 self.handle_key_event(key_event).wrap_err_with(|| {
//                     format!("handling key event failed:\n{key_event:#?}")
//                 })
//             }
//             _ => Ok(())
//         }
//     }

//     pub fn handle_key_event(&mut self, key_event: KeyEvent) -> Result<()> {
//         match key_event.code {
//             KeyCode::Char('q') => self.exit(),
//             _ => {}
//         }
//         Ok(())
//     }

//     fn exit(&mut self) {
//         self.exit = true;
//     }
// }