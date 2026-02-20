use anyhow::{Result, anyhow};

use crate::time::Time;

use super::Tui;
use super::pending_action::PendingAction;
use super::prompt::Prompt;
use super::selection::Level;

impl Tui {
    fn add_task(&self) -> Result<()> {
        if let Some(project_id) = self.state.selection.get_selected_project_id() {
            if let Prompt::Text((_, text, _)) = &self.state.prompt {
                self.storage
                    .add_task(project_id, &text.iter().collect::<String>(), None)
            } else {
                Err(anyhow!("error: unable to read prompt input"))
            }
        } else {
            Err(anyhow!("error: unable to get selected project id"))
        }
    }

    pub fn execute_pending_action(&self) -> Result<()> {
        match self.state.pending_action {
            PendingAction::Add => self.add_task(),
            PendingAction::Delete => self.delete_selected(),
            PendingAction::EditExpiration => self.submit_task_expiration(),
            PendingAction::EditName => self.submit_task_name(),
            PendingAction::None => Ok(()),
        }
    }

    fn delete_selected(&self) -> Result<()> {
        match self.state.selection.level {
            Level::Project => self.delete_selected_project(),
            Level::Task => self.delete_selected_task(),
        }
    }

    fn delete_selected_project(&self) -> Result<()> {
        if let Some(project_id) = self.state.selection.get_selected_project_id() {
            self.storage.delete_project(project_id)
        } else {
            Err(anyhow!("error: unable to get selected project id"))
        }
    }

    fn delete_selected_task(&self) -> Result<()> {
        if let Some(task_id) = self.state.selection.get_selected_task_id() {
            self.storage.delete_task(task_id)
        } else {
            Err(anyhow!("error: unable to get selected task id"))
        }
    }

    pub fn prompt_add(&mut self) {
        match self.state.selection.level {
            Level::Project => self.prompt_add_project(),
            Level::Task => self.prompt_add_task(),
        }
    }

    fn prompt_add_project(&mut self) {
        self.state.prompt =
            Prompt::Info("Cannot add project from TUI; use `task project new` in a directory");
    }

    fn prompt_add_task(&mut self) {
        self.state.prompt = Prompt::Text(("Enter name of task:", Vec::new(), 0));
    }

    pub fn prompt_delete_selected(&mut self) {
        match self.state.selection.level {
            Level::Project => self.prompt_delete_project(),
            Level::Task => self.prompt_delete_task(),
        }
    }

    fn prompt_delete_project(&mut self) {
        self.state.prompt = Prompt::Confirm("Are you sure you want to delete this project?");
    }

    fn prompt_delete_task(&mut self) {
        self.state.prompt = Prompt::Confirm("Are you sure you want to delete this task?");
    }

    pub fn prompt_edit_expiration(&mut self) -> Result<()> {
        if let Some(task_id) = self.state.selection.get_selected_task_id() {
            let task = self.storage.get_task(task_id)?;
            let text = task.expiration.map_or(String::new(), Time::format_relative);
            self.state.prompt = Prompt::Text((
                "Edit expiration time:",
                text.chars().collect::<Vec<char>>(),
                text.len(),
            ));
        }

        Ok(())
    }

    pub fn prompt_edit_name(&mut self) -> Result<()> {
        if let Some(task_id) = self.state.selection.get_selected_task_id() {
            let task = self.storage.get_task(task_id)?;
            self.state.prompt = Prompt::Text((
                "Edit name:",
                task.name.chars().collect::<Vec<char>>(),
                task.name.len(),
            ));
        }

        Ok(())
    }

    fn submit_task_name(&self) -> Result<()> {
        if let Some(task_id) = self.state.selection.get_selected_task_id() {
            if let Prompt::Text((_, text, _)) = &self.state.prompt {
                let task = self.storage.get_task(task_id)?;
                self.storage.update_task(
                    task_id,
                    Some(&text.iter().collect::<String>()),
                    task.expiration,
                )
            } else {
                Err(anyhow!("error: unable to read prompt input"))
            }
        } else {
            Err(anyhow!("error: unable to get selected task id"))
        }
    }

    fn submit_task_expiration(&self) -> Result<()> {
        if let Some(task_id) = self.state.selection.get_selected_task_id() {
            let task = self.storage.get_task(task_id)?;
            if let Prompt::Text((_, text, _)) = &self.state.prompt {
                self.storage.update_task(
                    task_id,
                    Some(&task.name),
                    Time::from_str(&text.iter().collect::<String>())?,
                )
            } else {
                Err(anyhow!("error: unable to read prompt input"))
            }
        } else {
            Err(anyhow!("error: unable to get selected task id"))
        }
    }

    pub fn toggle_finish(&self) {
        if let Some(task_id) = self.state.selection.get_selected_task_id() {
            let _ = self.storage.toggle_finish_task(task_id);
        }
    }
}
