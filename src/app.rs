use crate::storage::Storage;
use crate::io::TaskIO;
use crate::model::Task;
use crate::cli::Cli;
use anyhow::{anyhow, Result};

fn get_tasks(
    storage: &mut impl Storage,
    project: Option<String>,
) -> Result<&mut Vec<Task>> {
    let tasks = if let Some(name) = project {
        match storage.get_project_from_input(&name) {
            Some(t) => t,
            None => return Err(anyhow!("No project found with name '{name}'")),
        }
    } else {
        match storage.get_current_project()? {
            Some(t) => t,
            None => return Err(anyhow!("No project found in CWD")),
        }
    };

    Ok(tasks)
}

pub fn run(storage: &mut impl Storage, io: &mut impl TaskIO, cli: Cli) -> Result<()> {
    match cli.command {
        crate::cli::Commands::List { project } => {
            io.print_tasks(get_tasks(storage, project)?)?
        },
        crate::cli::Commands::Add { name, time, project } => {
            crate::app::actions::add_task(get_tasks(storage, project)?, &name, time.as_deref())?;
        }
        crate::cli::Commands::Delete { id, no_confirm, project } => {
            crate::app::actions::delete_task(get_tasks(storage, project)?, id, no_confirm, io)?;
        }
        crate::cli::Commands::Edit { id, name, time, project } => {
            crate::app::actions::edit_task(get_tasks(storage, project)?, id, name.as_deref(), time.as_deref())?;
        }
        crate::cli::Commands::Finish { id, project } => {
            crate::app::actions::finish_task(get_tasks(storage, project)?, id)?;
        }
    }

    storage.save()?;
    Ok(())
}

pub mod actions {
    use crate::model::{Task};
    use crate::utils::parse_to_unix;
    use anyhow::{Result, anyhow};
    use crate::io::TaskIO;

    pub fn add_task(tasks: &mut Vec<Task>, name: &str, expiration: Option<&str>) -> Result<()> {
        tasks.push(Task {
            id: tasks.len() as u32 + 1,
            name: name.to_string(),
            finished: false,
            expiration: expiration.and_then(parse_to_unix),
        });

        Ok(())
    }

    pub fn delete_task(tasks: &mut Vec<Task>, id: u32, no_confirm: bool, io: &mut impl TaskIO) -> Result<()> {
        let task = tasks.iter().find(|t| t.id == id)
            .ok_or_else(|| anyhow!("Task with id {} not found", id))?;

        if no_confirm || io.confirm_delete(&task.name)? {
            tasks.retain(|t| t.id != id);
            for (i, t) in tasks.iter_mut().enumerate() { t.id = i as u32 + 1; }
        }

        Ok(())
    }

    pub fn edit_task(tasks: &mut [Task], id: u32, name: Option<&str>, expiration: Option<&str>) -> Result<()> {
        let task = tasks.iter_mut().find(|t| t.id == id)
            .ok_or_else(|| anyhow!("Task with id {} not found", id))?;

        if let Some(n) = name { task.name = n.to_string(); }
        if let Some(exp) = expiration { task.expiration = parse_to_unix(exp); }

        Ok(())
    }

    pub fn finish_task(tasks: &mut [Task], id: u32) -> Result<()> {
        let task = tasks.iter_mut().find(|t| t.id == id)
            .ok_or_else(|| anyhow!("Task with id {} not found", id))?;
        task.finished = !task.finished;

        Ok(())
    }
}
