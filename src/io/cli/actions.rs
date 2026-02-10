use anyhow::Result;

use crate::time::Time;
use super::Cli;
use super::prompts;
use super::render;

impl Cli {
    pub fn new_project(&self) -> Result<()> {
        self.storage.new_project()?;
        render::render_new_project();

        Ok(())
    }

    pub fn list_projects(&self) -> Result<()> {
        let projects = self.storage.get_all_projects()?;
        render::render_projects(&projects);

        Ok(())
    }

    pub fn delete_project(&self, project_name: Option<String>) -> Result<()> {
        let project = self.resolve_project(project_name)?;
        match project {
            Some(project) => {
                if prompts::confirm_delete_project(&project)? {
                    self.storage.delete_project(project.id)?;
                }
            }
            None => render::render_project_not_found(),
        }

        Ok(())
    }

    pub fn list_tasks(&self, project_name: Option<String>, hide_finished: bool) -> Result<()> {
        let project = self.resolve_project(project_name)?;
        match project {
            Some(project) => render::render_tasks(&project, hide_finished),
            None => render::render_project_not_found(),
        }

        Ok(())
    }

    pub fn add_task(
        &self,
        project_name: Option<String>,
        name: &str,
        time: Option<&str>,
    ) -> Result<()> {
        let project = self.resolve_project(project_name)?;
        match project {
            Some(project) => {
                self.storage.add_task(project.id, name, time)?;
            }
            None => render::render_project_not_found(),
        }

        Ok(())
    }

    pub fn delete_task(
        &self,
        project_name: Option<String>,
        number: usize,
        no_confirm: bool,
    ) -> Result<()> {
        let project = self.resolve_project(project_name)?;
        match project {
            Some(project) => match self.resolve_task(project.id, number)? {
                Some(task) => {
                    if no_confirm || prompts::confirm_delete_task(&self.storage.get_task(task.id)?)?
                    {
                        self.storage.delete_task(task.id)?;
                    }
                }
                None => render::render_task_not_found(),
            },
            None => render::render_project_not_found(),
        }

        Ok(())
    }

    pub fn edit_task(
        &self,
        project_name: Option<String>,
        number: usize,
        name: Option<&str>,
        time: Option<&str>,
    ) -> Result<()> {
        let project = self.resolve_project(project_name)?;
        match project {
            Some(project) => match self.resolve_task(project.id, number)? {
                Some(task) => {
                    self.storage
                        .update_task(task.id, name, time.and_then(Time::from_str))?
                }
                None => render::render_task_not_found(),
            },
            None => render::render_project_not_found(),
        }

        Ok(())
    }

    pub fn finish_task(&self, project_name: Option<String>, number: usize) -> Result<()> {
        let project = self.resolve_project(project_name)?;
        match project {
            Some(project) => match self.resolve_task(project.id, number)? {
                Some(task) => self.storage.toggle_finish_task(task.id)?,
                None => render::render_task_not_found(),
            },
            None => render::render_project_not_found(),
        }

        Ok(())
    }
}
