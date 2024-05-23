mod activity_area_layout;
pub mod main_layout;
mod transaction_list;

use std::rc::Rc;

use color_eyre::eyre::Result;
use crossterm::event::{self, Event, KeyEvent, MouseEvent};
use ratatui::{layout::Rect, Frame};

pub trait Component {
    fn get_layout(&mut self, area: Rect) -> Rc<[Rect]> {
        [area].into()
    }
    fn render(&mut self, f: &mut Frame, area: Rect);
    fn handle_child_events(&mut self, event: &Event) -> Result<()> {
        Ok(())
    }
    fn handle_events(&mut self, event: &Event) -> Result<()> {
        match event {
            Event::Key(key_event) => self.handle_key_events(key_event),
            Event::Mouse(mouse_event) => self.handle_mouse_events(mouse_event),
            _ => {}
        };
        self.handle_child_events(event)
    }
    #[allow(unused_variables)]
    fn handle_key_events(&mut self, key: &KeyEvent) -> () {}
    #[allow(unused_variables)]
    fn handle_mouse_events(&mut self, mouse: &MouseEvent) -> () {}
}
