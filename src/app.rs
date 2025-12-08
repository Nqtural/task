use crate::storage::Storage;
use crate::io::TaskIO;
use crate::cli::Cli;
use anyhow::Result;

pub fn run(storage: &mut impl Storage, io: &mut impl TaskIO, cli: Cli) -> Result<()> {
    let mut tasks = storage.load()?;

    match cli.command {
        crate::cli::Commands::List => io.print_tasks(&tasks)?,
        crate::cli::Commands::Add { name, time } => {
            crate::app::actions::add_task(&mut tasks, &name, time.as_deref())?;
        }
        crate::cli::Commands::Delete { id, no_confirm } => {
            crate::app::actions::delete_task(&mut tasks, id, no_confirm, io)?;
        }
        crate::cli::Commands::Edit { id, name, time } => {
            crate::app::actions::edit_task(&mut tasks, id, name.as_deref(), time.as_deref())?;
        }
        crate::cli::Commands::Finish { id } => {
            crate::app::actions::finish_task(&mut tasks, id)?;
        }
    }

    storage.save(&tasks)?;
    Ok(())
}

pub mod actions {
    use crate::model::{Task, TaskList};
    use crate::utils::parse_to_unix;
    use anyhow::{Result, anyhow};
    use crate::io::TaskIO;

    pub fn add_task(tasks: &mut TaskList, name: &str, expiration: Option<&str>) -> Result<()> {
        tasks.tasks.push(Task {
            id: tasks.tasks.len() as u16 + 1,
            name: name.to_string(),
            finished: false,
            expiration: expiration.and_then(parse_to_unix),
        });
        Ok(())
    }

    pub fn delete_task(tasks: &mut TaskList, id: u16, no_confirm: bool, io: &mut impl TaskIO) -> Result<()> {
        let task = tasks.tasks.iter().find(|t| t.id == id)
            .ok_or_else(|| anyhow!("Task with id {} not found", id))?;

        if no_confirm || io.confirm_delete(&task.name)? {
            tasks.tasks.retain(|t| t.id != id);
            for (i, t) in tasks.tasks.iter_mut().enumerate() { t.id = i as u16 + 1; }
        }
        Ok(())
    }

    pub fn edit_task(tasks: &mut TaskList, id: u16, name: Option<&str>, expiration: Option<&str>) -> Result<()> {
        let task = tasks.tasks.iter_mut().find(|t| t.id == id)
            .ok_or_else(|| anyhow!("Task with id {} not found", id))?;

        if let Some(n) = name { task.name = n.to_string(); }
        if let Some(exp) = expiration { task.expiration = parse_to_unix(exp); }
        Ok(())
    }

    pub fn finish_task(tasks: &mut TaskList, id: u16) -> Result<()> {
        let task = tasks.tasks.iter_mut().find(|t| t.id == id)
            .ok_or_else(|| anyhow!("Task with id {} not found", id))?;
        task.finished = !task.finished;
        Ok(())
    }
}
