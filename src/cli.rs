use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "task")]
#[command(about = "Simple CLI task manager")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    List {
        project: Option<String>,
    },
    Add {
        name: String,
        #[arg(short, long)]
        time: Option<String>,
        project: Option<String>,
    },
    Delete {
        id: u32,
        #[arg(long, action = clap::ArgAction::SetTrue)]
        no_confirm: bool,
        project: Option<String>,
    },
    Edit {
        id: u32,
        #[arg(short, long)]
        name: Option<String>,
        #[arg(short, long)]
        time: Option<String>,
        project: Option<String>,
    },
    Finish {
        id: u32,
        project: Option<String>,
    },
}
