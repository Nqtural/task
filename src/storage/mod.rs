use crate::model::Project;
use anyhow::Result;

pub mod toml_storage;

pub trait Storage {
    fn tidy_path(&self, path: &str) -> String;
    fn new_project(&mut self) -> Result<()>;
    fn delete_project(&mut self, project_name: Option<String>) -> Result<()>;
    fn current_project_key(&self) -> Result<Option<String>>;
    fn get_project(&mut self, project_name: Option<String>, create_if_missing: bool) -> Result<&mut Project>;
    fn get_projects(&self) -> Vec<(String, usize)>;
    fn project_exists(&self, name: &str) -> bool;
    fn find_project_by_name(&mut self, project_name: &str) -> Option<&mut Project>;
    fn get_global_project(&mut self) -> &mut Project;
    fn get_current_project(&mut self) -> Result<Option<&mut Project>>;
    fn get_project_from_input(&mut self, project_name: &str) -> Option<&mut Project>;
    fn create_and_get_project(&mut self, project_name: &str) -> &mut Project;
    fn save(&self) -> Result<()>;
}
