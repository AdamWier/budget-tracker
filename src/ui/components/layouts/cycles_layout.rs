use std::rc::Rc;

use color_eyre::eyre::Result;
use crossterm::event::{Event, KeyCode, KeyEvent};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    Frame,
};

use crate::{
    csv::models::{list_item::ListItem, CycleFile},
    ui::{
        components::{reusable::scrollable_list::ScrollableList, Component},
        state::State,
    },
};

#[derive(Debug)]
pub struct CyclesLayout<'a> {
    cycle_list: ScrollableList<CycleFile>,
    state: &'a State,
}

impl CyclesLayout<'_> {
    pub fn init(state: &'_ State) -> CyclesLayout<'_> {
        CyclesLayout {
            cycle_list: ScrollableList::init(state.files.clone(), KeyCode::Up, KeyCode::Down),
            state,
        }
    }
    fn change_cycle(&self) {
        self.state.update_current_file(
            self.cycle_list
                .get_selected_item()
                .unwrap()
                .get_savable_value(),
        );
    }
    fn handle_enter_key(&mut self) {
        self.change_cycle()
    }
}

#[allow(clippy::single_match)]
impl Component<'_> for CyclesLayout<'_> {
    fn handle_key_events(&mut self, key_event: &KeyEvent) -> Result<()> {
        match key_event.code {
            KeyCode::Enter => self.handle_enter_key(),
            _ => {}
        }
        Ok(())
    }
    fn handle_child_events(&mut self, event: &Event) -> Result<()> {
        self.cycle_list.handle_events(event)?;
        Ok(())
    }
    fn get_layout(&self, area: Rect) -> Rc<[Rect]> {
        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(100)])
            .split(area)
    }
    fn render(&mut self, frame: &mut Frame<'_>, area: Rect) {
        let [layout] = *self.get_layout(area) else {
            panic!()
        };
        self.cycle_list.render(frame, layout);
    }
}
