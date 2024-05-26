use std::fmt::Debug;

use ratatui::text::Text;

pub trait ListItem {
    fn get_list_label(&self) -> Text;
    fn get_savable_value(&self) -> Vec<String>;
}

impl Debug for dyn ListItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ListItem{{{}}}", self.get_list_label())
    }
}
