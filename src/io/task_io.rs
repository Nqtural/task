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

        let id_width = project.tasks.iter().map(|t| t.id.to_string().len()).max().unwrap_or(0);
        let name_width = project.tasks.iter().map(|t| t.name.len()).max().unwrap_or(0);
        let last_width = project
            .tasks
            .iter()
            .map(|t| {
                if t.finished {
                    4 // evaluates to "DONE" later
                } else if let Some(exp) = t.expiration {
                    unix_to_relative(exp).len()
                } else {
                    0
                }
            })
        .max()
        .unwrap_or(0);

        println!("Listing tasks in project '{}'", project.path);

        for (index, task) in project.tasks.iter().enumerate() {
            if task.finished && hide_finished {
                continue;
            }

            let styled_name = if task.finished {
                task.name.bright_black().strikethrough()
            } else {
                task.name.white().bold()
            };

            let raw_last = if task.finished {
                "DONE".to_string()
            } else if let Some(exp) = task.expiration {
                unix_to_relative(exp)
            } else {
                "".to_string()
            };

            let styled_last = if task.finished {
                raw_last.green()
            } else if let Some(exp) = task.expiration {
                if exp - Utc::now().timestamp() <= 0 {
                    raw_last.red()
                } else {
                    raw_last.bright_black()
                }
            } else {
                raw_last.white()
            };

            print!("{: >id_width$}. ", index + 1, id_width = id_width + 1);
            print!("{}", styled_name);
            let name_pad = name_width.saturating_sub(task.name.len());
            print!("{:name_pad$} ", "", name_pad = name_pad);
            let last_pad = last_width.saturating_sub(raw_last.len());
            print!("{:last_pad$}", "", last_pad = last_pad);
            println!("{}", styled_last);
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
