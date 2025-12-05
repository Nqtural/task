use anyhow::{Result};
use std::{fs, path::Path};
use crate::entity::TaskList;

const FILE_NAME: &str = "tasks.toml";

// TODO
// - write_tasks function
// - file path logic

pub fn read_tasks(path: &Path) -> Result<TaskList> {
    let file_path = path.join(FILE_NAME);
    match fs::read_to_string(&file_path) {
        Ok(toml_str) => {
            let task_list: TaskList = toml::from_str(&toml_str)?;
            Ok(task_list)
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            // File doesn't exist – create an empty TaskList and write it
            let task_list = TaskList::new();
            let toml_str = toml::to_string_pretty(&task_list)?;
            fs::write(file_path, toml_str)?;
            Ok(task_list)
        }
        Err(e) => Err(e.into()),
    }
}
