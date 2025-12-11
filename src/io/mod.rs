use crate::model::Project;
use anyhow::Result;

pub mod cli_io;

pub trait TaskIO {
    fn new_project(&self);
    fn list_projects(&self, project_infos: Vec<(String, usize)>);
    fn print_tasks(&self, project: &Project) -> Result<()>;
    fn confirm_delete(&self, task_name: &str) -> Result<bool>;
}
