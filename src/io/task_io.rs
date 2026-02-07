use crate::types::{Project, Task};
use anyhow::Result;
use colored::*;
use chrono::Utc;
use crate::utils::unix_to_relative;
use std::io::{self, Write};

pub struct TaskIO;

impl TaskIO {
    pub fn new() -> Self {
        Self
    }
}

impl TaskIO {
    pub fn new_project(&self) {
        println!("Created new project");
    }

    pub fn list_projects(&self, projects: &[Project]) {
        let project_path_width = projects.iter().map(|p| p.path.len()).max().unwrap_or(1);

        println!("Projects:\n---------");
        for project in projects {
            println!(
                "{: <project_path_width$} ({} task{})",
                project.path,
                project.tasks.len(),
                if project.tasks.len() == 1 { "" } else { "s" }
            );
        }
    }

    pub fn print_tasks(&self, project: &Project, hide_finished: bool) -> Result<()> {
        if project.tasks.is_empty() {
            println!("No tasks yet. Create one with `task add \"My task\"`");
            return Ok(());
        }

        let id_width = project.tasks.iter().map(|t| t.id.to_string().len()).max().unwrap_or(1);
        let name_width = project.tasks.iter().map(|t| t.name.len()).max().unwrap_or(10);

        println!("Listing tasks in project '{}'", project.path);

        for (index, task) in project.tasks.iter().enumerate() {
            let last_column = if task.finished {
                if hide_finished { continue; }
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
            print!(
                "{: <name_width$} ",
                if task.finished { task.name.white() } else {task.name.cyan().bold() },
                name_width = name_width
            );
            println!("{: >15}", last_column);
        }

        Ok(())
    }

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

    pub fn project_not_found(&self) {
        println!("Project not found");
    }

    pub fn task_not_found(&self) {
        println!("Task not found");
    }
}
