use std::{
    path::Path,
    sync::{Arc, Mutex},
};

use anyhow::{Context, Result};
use notify::{ReadDirectoryChangesWatcher, RecursiveMode, Watcher};

use crate::csv::{
    models::{AssignedTransaction, BudgetItem, Transaction},
    parsers::assigned_transactions::parse_assigned_transactions_csv,
};

use super::{app::App, components::layouts::main_layout::MainLayout};

#[derive(Debug)]
pub struct State {
    pub transactions: Vec<Transaction>,
    pub blance: f32,
    pub budget_items: Vec<BudgetItem>,
    pub assigned_transactions: Arc<Mutex<Vec<AssignedTransaction>>>,
}

#[derive(Debug, Default)]
pub struct AppBuilder {
    assigned_transactions: Option<Arc<Mutex<Vec<AssignedTransaction>>>>,
    watcher: Option<ReadDirectoryChangesWatcher>,
}

impl<'a> AppBuilder {
    pub fn init() -> Self {
        Self {
            ..Default::default()
        }
    }
    pub fn create_watcher(mut self) -> Self {
        let clone = Arc::clone(self.assigned_transactions.as_ref().unwrap());
        let mut watcher: notify::ReadDirectoryChangesWatcher =
            notify::recommended_watcher(move |res| match res {
                Ok(_) => {
                    *clone.lock().unwrap() =
                        parse_assigned_transactions_csv("assigned_transactions.csv").unwrap()
                }
                Err(_) => panic!(),
            })
            .unwrap();
        watcher
            .watch(
                Path::new("assigned_transactions.csv"),
                RecursiveMode::Recursive,
            )
            .unwrap();
        self.watcher = Some(watcher);
        self
    }
    pub fn create_app(self, state: &'a State) -> Result<App<'a>> {
        let main_layout = MainLayout::init(state);

        Ok(App {
            exit: false,
            main_layout,
            watcher: self.watcher.context("No watcher created")?,
        })
    }
}
