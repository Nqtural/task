use anyhow::Result;

use crate::types::{Project, Task};
use super::Cli;

impl Cli {
    pub fn resolve_project(&self, input: Option<String>) -> Result<Option<Project>> {
        let id = match input {
            Some(name) => self.storage.find_project_by_dir_name(&name)?,
            None => self.storage.get_current_project()?,
        };

        match id {
            Some(id) => Ok(self.storage.get_project(id)?),
            None => Ok(None),
        }
    }

    pub fn resolve_task(&self, project_id: u32, number: usize) -> Result<Option<Task>> {
        Ok(
            self.storage
                .get_tasks(project_id)?
                // get number - 1 because task 0 is displayed as 1
                .get(number - 1)
                .and_then(|t| self.storage.get_task(t.id).ok())
        )
    }
}
