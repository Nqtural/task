use super::Storage;
use crate::model::TaskList;
use anyhow::Result;
use std::fs;
use std::path::PathBuf;

pub struct TomlStorage {
    path: PathBuf,
}

impl TomlStorage {
    pub fn new() -> Result<Self> {
        let base_dir = std::env::var("XDG_DATA_HOME")
            .unwrap_or_else(|_| format!("{}/.local/share", std::env::var("HOME").unwrap()));
        let path = PathBuf::from(base_dir).join("task");
        std::fs::create_dir_all(&path)?;
        Ok(Self { path })
    }
}

impl Storage for TomlStorage {
    fn load(&mut self) -> Result<TaskList> {
        let file_path = self.path.join("tasks.toml");
        match fs::read_to_string(&file_path) {
            Ok(s) => Ok(toml::from_str(&s)?),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                let tasks = TaskList { tasks: vec![] };
                self.save(&tasks)?;
                Ok(tasks)
            }
            Err(e) => Err(e.into()),
        }
    }

    fn save(&mut self, tasks: &TaskList) -> Result<()> {
        let toml_str = toml::to_string_pretty(tasks)?;
        fs::write(self.path.join("tasks.toml"), toml_str)?;
        Ok(())
    }
}
