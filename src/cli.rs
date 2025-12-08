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
    List,
    Add {
        name: String,
        #[arg(short, long)]
        time: Option<String>,
    },
    Delete {
        id: u16,
        #[arg(long, action = clap::ArgAction::SetTrue)]
        no_confirm: bool,
    },
    Edit {
        id: u16,
        #[arg(short, long)]
        name: Option<String>,
        #[arg(short, long)]
        time: Option<String>,
    },
    Finish {
        id: u16,
    },
}
