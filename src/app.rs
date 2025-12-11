use crate::storage::Storage;
use crate::io::TaskIO;
use crate::cli::{Cli, ProjectCommands};
use anyhow::Result;

pub fn run(storage: &mut impl Storage, io: &mut impl TaskIO, cli: Cli) -> Result<()> {
    match cli.command {
        crate::cli::Commands::Project { command } => {
            match command {
                ProjectCommands::New => {
                    storage.new_project()?;
                    io.new_project();
                },
                ProjectCommands::List => {
                    io.list_projects(storage.get_projects());
                },
                ProjectCommands::Delete { project } => {
                    storage.get_project(project.clone(), false)?;

                    if io.confirm_delete("project")? {
                        storage.delete_project(project)?;
                    }
                }
            }
        },
        crate::cli::Commands::List { project } => {
            io.print_tasks(storage.get_project(project, false)?)?
        },
        crate::cli::Commands::Add { name, time, project } => {
            crate::app::actions::add_task(storage.get_project(project, true)?, &name, time.as_deref())?;
        }
        crate::cli::Commands::Delete { id, no_confirm, project } => {
            crate::app::actions::delete_task(storage.get_project(project, false)?, id, no_confirm, io)?;
        }
        crate::cli::Commands::Edit { id, name, time, project } => {
            crate::app::actions::edit_task(storage.get_project(project, false)?, id, name.as_deref(), time.as_deref())?;
        }
        crate::cli::Commands::Finish { id, project } => {
            crate::app::actions::finish_task(storage.get_project(project, false)?, id)?;
        }
    }

    storage.save()?;
    Ok(())
}

pub mod actions {
    use crate::model::{Project, Task};
    use crate::utils::parse_to_unix;
    use anyhow::{Result, anyhow};
    use crate::io::TaskIO;

    pub fn add_task(project: &mut Project, name: &str, expiration: Option<&str>) -> Result<()> {
        project.tasks.push(Task {
            id: project.tasks.len() as u32 + 1,
            name: name.to_string(),
            finished: false,
            expiration: expiration.and_then(parse_to_unix),
        });

        Ok(())
    }

    pub fn delete_task(project: &mut Project, id: u32, no_confirm: bool, io: &mut impl TaskIO) -> Result<()> {
        let task = project.tasks.iter().find(|t| t.id == id)
            .ok_or_else(|| anyhow!("Task with id {} not found", id))?;

        if no_confirm || io.confirm_delete(&task.name)? {
            project.tasks.retain(|t| t.id != id);
            for (i, t) in project.tasks.iter_mut().enumerate() { t.id = i as u32 + 1; }
        }

        Ok(())
    }

    pub fn edit_task(project: &mut Project, id: u32, name: Option<&str>, expiration: Option<&str>) -> Result<()> {
        let task = project.tasks.iter_mut().find(|t| t.id == id)
            .ok_or_else(|| anyhow!("Task with id {} not found", id))?;

        if let Some(n) = name { task.name = n.to_string(); }
        if let Some(exp) = expiration { task.expiration = parse_to_unix(exp); }

        Ok(())
    }

    pub fn finish_task(project: &mut Project, id: u32) -> Result<()> {
        let task = project.tasks.iter_mut().find(|t| t.id == id)
            .ok_or_else(|| anyhow!("Task with id {} not found", id))?;
        task.finished = !task.finished;

        Ok(())
    }
}
