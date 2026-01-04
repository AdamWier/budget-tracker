use std::{fs::File, path::PathBuf, str::FromStr};

use anyhow::Result;
use chrono::Utc;

use crate::csv::models::list_item::ListItem;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

impl CycleFile {
    pub fn create_new_file() -> Result<CycleFile> {
        let now_string = Utc::now().format("%Y-%m-%d_%H-%M-%S");
        let list_label = format!("./cycles/{now_string}.csv");
        let path = PathBuf::from_str(list_label.as_str())?;
        File::create(&path)?;
        Ok(CycleFile { path, list_label })
    }
}
