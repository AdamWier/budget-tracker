use std::{
    path::Path,
    sync::{Arc, Mutex},
};

use anyhow::{Context, Result};
use notify::{ReadDirectoryChangesWatcher, RecursiveMode, Watcher};

use crate::csv::{
    models::{AssignedTransaction, BudgetItem, ParseResult, Transaction},
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
    transactions: Option<Vec<Transaction>>,
    balance: Option<f32>,
    budget_items: Option<Vec<BudgetItem>>,
    assigned_transactions: Option<Arc<Mutex<Vec<AssignedTransaction>>>>,
    watcher: Option<ReadDirectoryChangesWatcher>,
    state: Option<State>,
}

impl<'a> AppBuilder {
    pub fn init() -> Self {
        Self {
            ..Default::default()
        }
    }
    pub fn add_parse_result(mut self, parse_result: ParseResult) -> Self {
        self.transactions = Some(parse_result.transactions);
        self.balance = Some(parse_result.balance);
        self
    }
    pub fn add_budget_items(mut self, budget_items: Vec<BudgetItem>) -> Self {
        self.budget_items = Some(budget_items);
        self
    }
    pub fn add_assigned_transactions(
        mut self,
        assigned_transactions: Vec<AssignedTransaction>,
    ) -> Self {
        let assigned_transactions_arc = Arc::new(Mutex::new(assigned_transactions));
        self.assigned_transactions = Some(assigned_transactions_arc);
        self
    }
    pub fn create_watcher(mut self) -> Self {
        let clone = Arc::clone(&self.assigned_transactions.as_ref().unwrap());
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
    pub fn create_state(&self) -> Result<State> {
        let state = State {
            assigned_transactions: self
                .assigned_transactions
                .clone()
                .context("No assigned transactions")?,
            transactions: self.transactions.clone().context("No transactions")?,
            blance: self.balance.context("No balance")?,
            budget_items: self.budget_items.clone().context("No budget items")?,
        };
        Ok(state)
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
