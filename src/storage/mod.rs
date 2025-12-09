use crate::model::Task;
use anyhow::Result;

pub mod toml_storage;

pub trait Storage {
    fn project_exists(&self, name: &str) -> bool;
    fn find_project_name(&self) -> Result<Option<String>>;
    fn find_project_by_name(&mut self, project_name: &str) -> Option<&mut Vec<Task>>;
    fn get_global_project(&mut self) -> &mut Vec<Task>;
    fn get_current_project(&mut self) -> Result<Option<&mut Vec<Task>>>;
    fn get_project_from_input(&mut self, project_name: &str) -> Option<&mut Vec<Task>>;
    fn create_and_get_project(&mut self, project_name: &str) -> &mut Vec<Task>;
    fn save(&self) -> Result<()>;
}
