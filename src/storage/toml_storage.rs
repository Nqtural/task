use super::Storage;
use crate::model::{Root, Task, Project};
use anyhow::Result;
use std::fs;
use std::path::PathBuf;

pub struct TomlStorage {
    path: PathBuf,
    pub root: Root,
}

impl TomlStorage {
    pub fn new() -> Result<Self> {
        let base_dir = std::env::var("XDG_DATA_HOME")
            .unwrap_or_else(|_| format!("{}/.local/share", std::env::var("HOME").unwrap()));
        let path = PathBuf::from(base_dir).join("task");
        std::fs::create_dir_all(&path)?;

        let file_path = path.join("tasks.toml");
        let root = if file_path.exists() {
            let s = fs::read_to_string(&file_path)?;
            toml::from_str(&s)?
        } else {
            Root::default()
        };

        Ok(Self { path, root })
    }
}

impl Storage for TomlStorage {
    fn find_project_name(&self) -> Result<Option<String>> {
        let mut cwd = std::env::current_dir()?;
        loop {
            if let Some(name_os) = cwd.file_name() {
                let name = name_os.to_string_lossy().to_string();
                if self.root.projects.contains_key(&name) {
                    return Ok(Some(name));
                }
            }
            cwd = match cwd.parent() {
                Some(parent) => parent.to_path_buf(),
                None => return Ok(None),
            };
        }
    }

    fn project_exists(&self, name: &str) -> bool {
        self.root.projects.contains_key(name)
    }

    fn find_project_by_name(&mut self, project_name: &str) -> Option<&mut Vec<Task>> {
        self.root
            .projects
            .get_mut(project_name)
            .map(|project| &mut project.tasks)
    }

    fn get_global_project(&mut self) -> &mut Vec<Task> {
        &mut self.root.global.tasks
    }

    fn get_current_project(&mut self) -> Result<Option<&mut Vec<Task>>> {
        let name = self.find_project_name()?;

        if let Some(name) = name {
            Ok(self.find_project_by_name(&name))
        } else {
            Ok(Some(self.get_global_project()))
        }
    }

    fn get_project_from_input(&mut self, project_name: &str) -> Option<&mut Vec<Task>> {
        if project_name.eq_ignore_ascii_case("global") {
            Some(self.get_global_project())
        } else {
            self.find_project_by_name(project_name)
        }

    }

    fn create_and_get_project(&mut self, name: &str) -> &mut Vec<Task> {
        self.root.projects.insert(name.to_string(), Project::default());
        self.root.projects.get_mut(name).unwrap().tasks.as_mut()

    }

    fn save(&self) -> Result<()> {
        let toml_str = toml::to_string_pretty(&self.root)?;
        fs::write(self.path.join("tasks.toml"), toml_str)?;
        Ok(())
    }
}
