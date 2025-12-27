use std::{path::PathBuf, rc::Rc};

use color_eyre::eyre::Result;
use crossterm::event::{Event, KeyCode, KeyEvent};
use itertools::Itertools;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    Frame,
};

use crate::{
    csv::models::list_item::ListItem,
    ui::{
        components::{reusable::scrollable_list::ScrollableList, Component},
        state::State,
    },
};

#[derive(Debug)]
pub struct CyclesLayout<'a> {
    cycle_list: ScrollableList,
    state: &'a State,
}

impl ListItem for PathBuf {
    fn get_list_label(&self) -> ratatui::prelude::Text {
        ratatui::text::Text::raw(self.file_name().unwrap().to_str().unwrap().to_string())
    }
    fn get_savable_value(&self) -> Vec<String> {
        vec![self.file_name().unwrap().to_str().unwrap().to_string()]
    }
}

impl CyclesLayout<'_> {
    pub fn init(state: &'_ State) -> CyclesLayout<'_> {
        let cycle_file_names = state
            .files
            .clone()
            .into_iter()
            .map(|x| Box::new(x) as Box<dyn ListItem>)
            .collect_vec();

        CyclesLayout {
            cycle_list: ScrollableList::init(cycle_file_names, KeyCode::Up, KeyCode::Down),
            state,
        }
    }
    fn change_cycle(&mut self) {
        *self.state.currently_selected_files.lock().unwrap() = PathBuf::from(
            self.cycle_list
                .get_selected_item()
                .unwrap()
                .get_savable_value()
                .first()
                .unwrap(),
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
