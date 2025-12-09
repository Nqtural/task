use crate::model::Task;
use anyhow::Result;

pub mod cli_io;

pub trait TaskIO {
    fn print_tasks(&mut self, tasks: &[Task]) -> Result<()>;
    fn confirm_delete(&mut self, task_name: &str) -> Result<bool>;
}
