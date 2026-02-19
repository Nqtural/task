mod io;
mod storage;
mod time;
mod types;

use anyhow::Result;
use clap::Parser;

fn main() -> Result<()> {
    let args = io::Args::parse();
    let storage = storage::Storage::new()?;
    match args.command {
        io::Commands::Tui => io::Tui::new(storage)?.run(),
        _ => io::Cli::new(storage).run(args),
    }
}
