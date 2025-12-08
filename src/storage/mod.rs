use crate::model::TaskList;
use anyhow::Result;

pub mod toml_storage;

pub trait Storage {
    fn load(&mut self) -> Result<TaskList>;
    fn save(&mut self, tasks: &TaskList) -> Result<()>;
}
