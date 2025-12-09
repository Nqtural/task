mod app;
mod cli;
mod io;
mod storage;
mod model;
mod utils;

use anyhow::Result;
use clap::Parser;

fn main() -> Result<()> {
    // Initialize CLI
    let cli = cli::Cli::parse();

    // Pick I/O layer (CLI or TUI)
    let mut io = io::cli_io::CliIO::new(); // swap to io::tui_io::TuiIO later

    // Pick Storage layer (TOML or SQLite)
    let mut storage = storage::toml_storage::TomlStorage::new()?; // swap to sqlite_storage

    // Run app
    app::run(&mut storage, &mut io, cli)
}
