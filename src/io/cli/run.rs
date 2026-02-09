use anyhow::Result;

use crate::io::{Args, Commands, ProjectCommands};
use super::Cli;

impl Cli {
    pub fn run(&self, args: Args) -> Result<()> {
        match args.command {
            Commands::Project { command } => {
                match command {
                    ProjectCommands::New => { self.new_project()?; },
                    ProjectCommands::List => { self.list_projects()?; },
                    ProjectCommands::Delete { project } => { self.delete_project(project)?; }
                }
            },
            Commands::List { project, hide_finished } => {
                self.list_tasks(project, hide_finished)?;
            },
            Commands::Add { name, time, project } => {
                self.add_task(project, &name, time.as_deref())?;
            },
            Commands::Delete { number, no_confirm, project } => {
                self.delete_task(project, number, no_confirm)?;
            },
            Commands::Edit { number, name, time, project } => {
                self.edit_task(project, number, name.as_deref(), time.as_deref())?;
            },
            Commands::Finish { number, project } => {
                self.finish_task(project, number)?;
            },
            Commands::Tui => {},
        }

        Ok(())
    }
}
