use std::fmt::Debug;

use ratatui::text::Text;

pub trait ListItem<T: PartialEq> {
    fn get_list_label(&self) -> Text;
    fn get_savable_value(&self) -> T;
}

impl<T: PartialEq> Debug for dyn ListItem<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ListItem{{{}}}", self.get_list_label())
    }
}
