use crate::io::TaskIO;
use crate::model::{Project, Task};
use anyhow::Result;
use colored::*;
use chrono::Utc;
use crate::utils::unix_to_relative;
use std::io::{self, Write};

pub struct CliIO;

impl CliIO {
    pub fn new() -> Self {
        Self
    }
}

impl TaskIO for CliIO {
    fn new_project(&self) {
        println!("Created new project");
    }

    fn list_projects(&self, project_infos: Vec<(String, usize)>) {
        let project_name_width = project_infos.iter().map(|p| p.0.len()).max().unwrap_or(1);

        println!("Projects:\n---------");
        for project_info in &project_infos {
            println!(
                "{: <project_name_width$} ({} task{})",
                project_info.0,
                project_info.1,
                if project_info.1 == 1 { "" } else { "s" }
            );
        }
    }

    fn print_tasks(&self, project: &Project) -> Result<()> {
        if project.tasks.is_empty() {
            println!("No tasks yet. Create one with `task add \"My task\"`");
            return Ok(());
        }

        let id_width = project.tasks.iter().map(|t| t.id.to_string().len()).max().unwrap_or(1);
        let name_width = project.tasks.iter().map(|t| t.name.len()).max().unwrap_or(10);

        for (index, task) in project.tasks.iter().enumerate() {
            let last_column = if task.finished {
                "DONE".green()
            } else if let Some(exp) = task.expiration {
                if exp - Utc::now().timestamp() <= 0 {
                    unix_to_relative(exp).red()
                } else {
                    unix_to_relative(exp).bright_black()
                }
            } else {
                "".white()
            };

            print!("{: >id_width$}. ", index + 1, id_width = id_width + 1);
            print!("{: <name_width$} ", task.name.cyan().bold(), name_width = name_width);
            println!("{: >15}", last_column);
        }

        Ok(())
    }

    fn confirm_delete_project(&self, project: &Project) -> Result<bool> {
        print!(
            "Are you sure you want to delete project '{}'? (contains {} task{}) (y/N): ",
            project.name,
            project.tasks.len(),
            if project.tasks.len() == 1 { "" } else { "s"},
        );
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        Ok(input.to_lowercase().contains('y'))
    }

    fn confirm_delete_task(&self, task: &Task) -> Result<bool> {
        print!("Are you sure you want to delete task '{}'? (y/N): ", task.name);
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        Ok(input.to_lowercase().contains('y'))
    }
}
