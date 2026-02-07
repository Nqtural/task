use anyhow::Result;

use crate::cli::{Cli, Commands, ProjectCommands};
use crate::io::TaskIO;
use crate::storage::TaskStorage;
use crate::utils::parse_to_unix;

fn get_project_id_from_input_or_current(storage: &TaskStorage, input: Option<String>) -> Result<Option<u32>> {
    Ok(match input {
        Some(input) => storage.find_project_by_dir_name(&input)?,
        None => storage.get_current_project()?,
    })
}

pub fn run(storage: &TaskStorage, io: &mut TaskIO, cli: Cli) -> Result<()> {
    match cli.command {
        Commands::Project { command } => {
            match command {
                ProjectCommands::New => {
                    storage.new_project()?;
                    io.new_project();
                },
                ProjectCommands::List => {
                    io.list_projects(&storage.get_all_projects()?);
                },
                ProjectCommands::Delete { project } => {
                    let project_id = get_project_id_from_input_or_current(storage, project)?;

                    match project_id {
                        Some(project_id) => {
                            // unwrap is safe because project_id exists
                            let project = storage.get_project(project_id)?.unwrap();

                            if io.confirm_delete_project(&project)? {
                                storage.delete_project(project_id)?;
                            }
                        },
                        None => io.project_not_found(),
                    }
                }
            }
        },
        Commands::List { project, hide_finished } => {
            let project_id = get_project_id_from_input_or_current(storage, project)?;
            match project_id {
                Some(project_id) => {
                    // unwrap is safe because project_id exists
                    let project = storage.get_project(project_id)?.unwrap();
                    io.print_tasks(&project, hide_finished)?;
                },
                None => io.project_not_found(),
            }
        },
        Commands::Add { name, time, project } => {
            let project_id = get_project_id_from_input_or_current(storage, project)?;
            match project_id {
                Some(project_id) => {
                    storage.add_task(
                        project_id,
                        &name,
                        time.as_deref(),
                    )?;
                },
                None => io.project_not_found(),
            }
        },
        Commands::Delete { number, no_confirm, project } => {
            let project_id = get_project_id_from_input_or_current(storage, project)?;
            match project_id {
                Some(project_id) => {
                    // get number - 1 because task 0 is displayed as 1
                    match storage.get_tasks(project_id)?.get(number - 1) {
                        Some(task) => {
                            if no_confirm || io.confirm_delete_task(&storage.get_task(task.id)?)? {
                                storage.delete_task(task.id)?;
                            }
                        },
                        None => io.task_not_found(),
                    }
                }
                None => io.project_not_found(),
            }
        },
        Commands::Edit { number, name, time, project } => {
            let project_id = get_project_id_from_input_or_current(storage, project)?;
            match project_id {
                Some(project_id) => {
                    // get number - 1 because task 0 is displayed as 1
                    match storage.get_tasks(project_id)?.get(number - 1) {
                        Some(task) => storage.update_task(
                            task.id,
                            name.as_deref(),
                            time.as_deref().and_then(parse_to_unix),
                        )?,
                        None => io.task_not_found(),
                    }
                },
                None => io.project_not_found(),
            }
        },
        Commands::Finish { number, project } => {
            let project_id = get_project_id_from_input_or_current(storage, project)?;
            match project_id {
                Some(project_id) => {
                    // get number - 1 because task 0 is displayed as 1
                    match storage.get_tasks(project_id)?.get(number - 1) {
                        Some(task) => storage.toggle_finish_task(task.id)?,
                        None => io.task_not_found(),
                    }
                },
                None => io.project_not_found(),
            }
        }
    }

    Ok(())
}
