use std::{
    path::Path,
    sync::{Arc, Mutex},
};

use anyhow::{Context, Result};
use notify::{ReadDirectoryChangesWatcher, RecursiveMode, Watcher};

use crate::{
    consts::CYCLES_FOLDER,
    csv::{
        models::{AssignedTransaction, CycleFile},
        parsers::assigned_transactions::parse_assigned_transactions_csv,
    },
    start_up::get_file_list,
};

use super::{app::App, components::layouts::main_layout::MainLayout, state::State};

#[derive(Debug, Default)]
pub struct AppBuilder {
    assigned_transactions_watcher: Option<ReadDirectoryChangesWatcher>,
    cycle_files_watcher: Option<ReadDirectoryChangesWatcher>,
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
        self.assigned_transactions_watcher = Some(watcher);
        self
    }
    pub fn create_cycle_file_watcher(mut self, cycle_files: &Arc<Mutex<Vec<CycleFile>>>) -> Self {
        let cycle_files_clone = cycle_files.clone();
        let mut watcher: notify::ReadDirectoryChangesWatcher =
            notify::recommended_watcher(move |res| match res {
                Ok(_) => *cycle_files_clone.lock().unwrap() = get_file_list().unwrap(),
                Err(_) => panic!(),
            })
            .unwrap();
        watcher
            .watch(Path::new(CYCLES_FOLDER), RecursiveMode::Recursive)
            .unwrap();
        self.cycle_files_watcher = Some(watcher);
        self
    }
    pub fn create_app(self, state: &'a State) -> Result<App<'a>> {
        let main_layout = MainLayout::init(state);

        Ok(App::new(
            main_layout,
            self.assigned_transactions_watcher
                .context("No assigned files watcher created")?,
            self.cycle_files_watcher
                .context("No cycle files watcher created")?,
        ))
    }
}
