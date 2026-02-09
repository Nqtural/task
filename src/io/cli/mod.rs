mod actions;
mod prompts;
mod render;
mod resolve;
mod run;

use crate::storage::TaskStorage;

pub struct Cli {
    storage: TaskStorage,
}

impl Cli {
    pub fn new(storage: TaskStorage) -> Self {
        Self { storage }
    }
}
