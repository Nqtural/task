mod io;
mod storage;
mod types;
mod utils;

use anyhow::Result;
use clap::Parser;

fn main() -> Result<()> {
    let args = io::Args::parse();
    let storage = storage::TaskStorage::new()?;
    match args.command {
        io::Commands::Tui => todo!(),
        _ => io::Cli::new(storage).run(args),
    }
}
