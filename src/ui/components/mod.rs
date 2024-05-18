pub mod layout;

use ratatui::Frame;

pub trait Component {
    fn render(&mut self, f: &mut Frame);
}