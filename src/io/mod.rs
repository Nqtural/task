use crate::model::{Project, Task};
use anyhow::Result;

pub mod cli_io;

pub trait TaskIO {
    fn new_project(&self);
    fn list_projects(&self, project_infos: Vec<(String, usize)>);
    fn print_tasks(&self, project: &Project) -> Result<()>;
    fn confirm_delete_project(&self, project: &Project) -> Result<bool>;
    fn confirm_delete_task(&self, task: &Task) -> Result<bool>;
}
