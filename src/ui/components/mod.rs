pub mod layout;

use std::rc::Rc;

use ratatui::{layout::Rect, Frame};

pub trait Component {
    fn get_layout(&mut self, frame: &mut Frame) -> Rc<[Rect]>;
    fn render(&mut self, f: &mut Frame);
}