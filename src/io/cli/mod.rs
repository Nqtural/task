mod actions;
mod prompts;
mod render;
mod resolve;
mod run;

use crate::storage::Storage;

pub struct Cli {
    storage: Storage,
}

impl Cli {
    pub fn new(storage: Storage) -> Self {
        Self { storage }
    }
}
