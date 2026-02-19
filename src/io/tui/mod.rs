mod action;
mod input_handler;
mod run;
mod selection;
mod state;
mod ui;

use anyhow::Result;

use crate::storage::Storage;
use state::State;
use ui::Ui;

pub struct Tui {
    state: State,
    storage: Storage,
    ui: Ui,
}

impl Tui {
    pub fn new(storage: Storage) -> Result<Self> {
        Ok(Self {
            state: State::new(),
            storage,
            ui: Ui::new()?,
        })
    }
}
