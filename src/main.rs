mod app;
mod cli;
mod io;
mod storage;
mod types;
mod utils;

use anyhow::Result;
use clap::Parser;

fn main() -> Result<()> {
    let cli = cli::Cli::parse();
    let mut io = io::TaskIO::new();
    let storage = storage::TaskStorage::new()?;
    app::run(&storage, &mut io, cli)
}
