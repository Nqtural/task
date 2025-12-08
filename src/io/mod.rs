use crate::model::TaskList;
use anyhow::Result;

pub mod cli_io;

pub trait TaskIO {
    fn print_tasks(&mut self, tasks: &TaskList) -> Result<()>;
    fn confirm_delete(&mut self, task_name: &str) -> Result<bool>;
}
