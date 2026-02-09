use anyhow::Result;
use std::io::{self, Write};

use crate::types::{Project, Task};
use super::Cli;

impl Cli {
    pub fn confirm_delete_project(&self, project: &Project) -> Result<bool> {
        print!(
            "Are you sure you want to delete project '{}'? (contains {} task{}) (y/N): ",
            project.path,
            project.tasks.len(),
            if project.tasks.len() == 1 { "" } else { "s"},
        );
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        Ok(input.to_lowercase().contains('y'))
    }

    pub fn confirm_delete_task(&self, task: &Task) -> Result<bool> {
        print!("Are you sure you want to delete task '{}'? (y/N): ", task.name);
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        Ok(input.to_lowercase().contains('y'))
    }
}
