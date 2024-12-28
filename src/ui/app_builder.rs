use std::{
    path::Path,
    sync::{Arc, Mutex},
};

use anyhow::{Context, Result};
use notify::{ReadDirectoryChangesWatcher, RecursiveMode, Watcher};

use crate::{
    consts::ASSIGNED_TRANSACTIONS_FILE_NAME,
    csv::{
        models::AssignedTransaction,
        parsers::assigned_transactions::parse_assigned_transactions_csv,
    },
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
    pub fn create_watcher(
        mut self,
        assigned_transactions: &Arc<Mutex<Vec<AssignedTransaction>>>,
    ) -> Self {
        let clone = Arc::clone(assigned_transactions);
        let mut watcher: notify::ReadDirectoryChangesWatcher =
            notify::recommended_watcher(move |res| match res {
                Ok(_) => {
                    *clone.lock().unwrap() =
                        parse_assigned_transactions_csv(ASSIGNED_TRANSACTIONS_FILE_NAME).unwrap()
                }
                Err(_) => panic!(),
            })
            .unwrap();
        watcher
            .watch(
                Path::new(ASSIGNED_TRANSACTIONS_FILE_NAME),
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
