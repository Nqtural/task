use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "task")]
#[command(about = "Simple CLI task manager")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum ProjectCommands {
    New,
    List,
    Delete {
        project: Option<String>,
    },
}

#[derive(Subcommand)]
pub enum Commands {
    Project {
        #[command(subcommand)]
        command: ProjectCommands,
    },
    List {
        project: Option<String>,
        #[arg(long, action = clap::ArgAction::SetTrue)]
        hide_finished: bool,
    },
    Add {
        name: String,
        #[arg(short, long)]
        time: Option<String>,
        project: Option<String>,
    },
    Delete {
        number: usize,
        #[arg(long, action = clap::ArgAction::SetTrue)]
        no_confirm: bool,
        project: Option<String>,
    },
    Edit {
        number: usize,
        #[arg(short, long)]
        name: Option<String>,
        #[arg(short, long)]
        time: Option<String>,
        project: Option<String>,
    },
    Finish {
        number: usize,
        project: Option<String>,
    },
}
