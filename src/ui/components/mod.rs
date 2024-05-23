mod activity_area_layout;
pub mod main_layout;
mod transaction_list;

use std::rc::Rc;

use crossterm::event::{Event, KeyEvent, MouseEvent};
use ratatui::{layout::Rect, Frame};

pub trait Component {
    fn get_layout(&mut self, area: Rect) -> Rc<[Rect]>;
    fn render(&mut self, f: &mut Frame, area: Rect);
    fn handle_events(&mut self, event: &Event) -> () {
        match event {
            Event::Key(key_event) => self.handle_key_events(key_event),
            Event::Mouse(mouse_event) => self.handle_mouse_events(mouse_event),
            _ => {}
        };
    }
    #[allow(unused_variables)]
    fn handle_key_events(&mut self, key: &KeyEvent) -> () {}
    #[allow(unused_variables)]
    fn handle_mouse_events(&mut self, mouse: &MouseEvent) -> () {}
}
