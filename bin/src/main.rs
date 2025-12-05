use clap::{Parser, Subcommand};
use std::{env, fs, path::Path};
use anyhow::{Result, anyhow};

// TODO
// - extract write/save tasks to model crate
// - extract file path logic to model crate

#[derive(Parser)]
#[command(name = "task")]
#[command(about = "Simple CLI task manager")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// List tasks
    List,
    /// Add a new task
    Add {
        name: String,
        #[arg(short, long)]
        time: Option<String>,
    },
    /// Delete a task by ID (use --no-confirm to skip prompt)
    Delete {
        id: u16,
        #[arg(long, action = clap::ArgAction::SetTrue)]
        no_confirm: bool,
    },
    /// Edit a task (use --name or -n for name and --time or -t for expire time)
    Edit {
        id: u16,
        #[arg(short, long)]
        name: Option<String>,
        #[arg(short, long)]
        time: Option<String>,
    },
    /// Toggle finished status
    Finish {
        id: u16,
    },
}

fn run() -> Result<()> {
    let base_dir = env::var("XDG_DATA_HOME")
        .unwrap_or_else(|_| format!("{}/.local/share", env::var("HOME").unwrap()));
    let path = Path::new(&base_dir).join("task/");
    
    if !path.exists() {
        fs::create_dir_all(&path)?;
    }

    let cli = Cli::parse();
    let mut task_vec = model::read_tasks(&path)?;

    match cli.command {
        Commands::List => {
            view::print_tasks(&task_vec)?;
        }
        Commands::Add { name, time } => {
            controller::add_task(&mut task_vec, &name, time.as_deref().unwrap_or(""))?;
        }
        Commands::Delete { id, no_confirm } => {
            controller::delete_task(&mut task_vec, id, no_confirm)?;
        }
        Commands::Edit { id, name, time } => {
            if name.is_none() && time.is_none() {
                return Err(anyhow!("--edit requires either '--name <NAME>' or '--time <TIME>'"));
            }
            controller::edit_task(&mut task_vec, id, name.as_deref(), time.as_deref())?;
        }
        Commands::Finish { id } => {
            controller::finish_task(&mut task_vec, id)?;
        }
    }

    let toml_str = toml::to_string_pretty(&task_vec)?;
    fs::write(path.join("tasks.toml"), toml_str)?;

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
    }
}
