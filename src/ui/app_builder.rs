use std::sync::{Arc, Mutex};

use anyhow::{Context, Result};
use notify::{ReadDirectoryChangesWatcher, RecursiveMode, Watcher};

use crate::csv::{
    models::{AssignedTransaction, CycleFile},
    parsers::assigned_transactions::parse_assigned_transactions_csv,
};

use super::{app::App, components::layouts::main_layout::MainLayout, state::State};

#[derive(Debug, Default)]
pub struct AppBuilder {
    watcher: Option<ReadDirectoryChangesWatcher>,
}

impl<'a> AppBuilder {
    pub fn init() -> Self {
        Self {
            ..Default::default()
        }
    }
    pub fn create_assigned_transaction_watcher(
        mut self,
        assigned_transactions: &Arc<Mutex<Vec<AssignedTransaction>>>,
        file: &Arc<Mutex<CycleFile>>,
    ) -> Self {
        let clone = Arc::clone(assigned_transactions);
        let file_clone = file.clone();
        let second_file_clone = file.clone();
        let mut watcher: notify::ReadDirectoryChangesWatcher =
            notify::recommended_watcher(move |res| match res {
                Ok(_) => {
                    *clone.lock().unwrap() =
                        parse_assigned_transactions_csv(&file_clone.lock().unwrap().path).unwrap()
                }
                Err(_) => panic!(),
            })
            .unwrap();
        watcher
            .watch(
                &second_file_clone.lock().unwrap().path,
                RecursiveMode::Recursive,
            )
            .unwrap();
        self.watcher = Some(watcher);
        self
    }
    pub fn create_app(self, state: &'a State) -> Result<App<'a>> {
        let main_layout = MainLayout::init(state);

        Ok(App::new(
            main_layout,
            self.watcher.context("No watcher created")?,
        ))
    }
}
