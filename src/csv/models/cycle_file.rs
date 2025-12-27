use std::path::PathBuf;

use crate::csv::models::list_item::ListItem;

#[derive(Debug, Clone, PartialEq)]
pub struct CycleFile {
    pub path: PathBuf,
    pub list_label: String,
}

impl ListItem<CycleFile> for CycleFile {
    fn get_list_label(&self) -> ratatui::prelude::Text {
        ratatui::text::Text::raw(self.list_label.clone())
    }
    fn get_savable_value(&self) -> CycleFile {
        self.clone()
    }
}
