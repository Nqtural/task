use anyhow::Result;
use rusqlite::Connection;
use std::path::PathBuf;

pub struct TaskStorage {
    pub conn: Connection,
}

impl TaskStorage {
    pub fn new() -> Result<Self> {
        let conn = Connection::open(get_storage_path()?)?;
        conn.execute(
            "PRAGMA foreign_keys = ON",
            [],
        )?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS projects(
                id INTEGER PRIMARY KEY,
                path TEXT NOT NULL UNIQUE
            )",
            [],
        )?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS tasks(
                id INTEGER PRIMARY KEY,
                project_id INTEGER NOT NULL,
                name TEXT NOT NULL,
                finished INTEGER NOT NULL,
                expiration INTEGER,
                FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE
            )",
            [],
        )?;

        Ok(Self { conn })
    }
}

fn get_storage_path() -> Result<String> {
    let base_dir = std::env::var("XDG_DATA_HOME")
        .unwrap_or_else(|_| format!("{}/.local/share", std::env::var("HOME").unwrap()));
    let path = PathBuf::from(base_dir).join("task");
    std::fs::create_dir_all(&path)?;

    Ok(path.join("task.db").to_string_lossy().to_string())
}
