pub mod layouts;
mod reusable;

use std::rc::Rc;

use color_eyre::eyre::{Context, Result};
use crossterm::event::{Event, KeyEvent, KeyEventKind, MouseEvent};
use ratatui::{layout::Rect, Frame};

pub trait Component<'a> {
    fn render(&mut self, f: &mut Frame, area: Rect);
    fn get_layout(&self, area: Rect) -> Rc<[Rect]> {
        [area].into()
    }
    #[allow(unused_variables)]
    fn handle_child_events(&mut self, event: &Event) -> Result<()> {
        Ok(())
    }
    fn handle_events(&mut self, event: &Event) -> Result<()> {
        match event {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                { self.handle_key_events(key_event) }
                    .wrap_err_with(|| format!("handling key event failed:\n{key_event:#?}"))
            }
            Event::Mouse(mouse_event) => self.handle_mouse_events(mouse_event),
            _ => Ok(()),
        }?;
        self.handle_child_events(event)
    }
    #[allow(unused_variables)]
    fn handle_key_events(&mut self, key: &KeyEvent) -> Result<()> {
        Ok(())
    }
    #[allow(unused_variables)]
    fn handle_mouse_events(&mut self, mouse: &MouseEvent) -> Result<()> {
        Ok(())
    }
}
