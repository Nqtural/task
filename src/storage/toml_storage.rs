use super::Storage;
use crate::model::{Root, Project};
use anyhow::{anyhow, Result};
use std::fs;
use std::path::PathBuf;
use dirs::home_dir;

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
    fn tidy_path(&self, path: &str) -> String {
        if let Some(home) = home_dir()
        && let Some(home_str) = home.to_str() {
            return path.replacen(home_str, "~", 1);
        }

        path.to_string()
    }

    fn new_project(&mut self) -> Result<()> {
        let cwd = std::env::current_dir()?;
        self.root.projects.insert(
            cwd.to_str().unwrap().to_string(),
            Project::default()
        );

        Ok(())
    }

    fn delete_project(&mut self, project_name: Option<String>) -> Result<()> {
        match project_name {
            Some(name) => self.root.projects.retain(|k, _| !k.ends_with(&name)),
            None => if let Some(key) = self.current_project_key()? {
                self.root.projects.remove(&key);
            }
        }

        Ok(())
    }

    fn current_project_key(&self) -> Result<Option<String>> {
        let mut cwd = std::env::current_dir()?.canonicalize()?;

        loop {
            let key = cwd.to_string_lossy().to_string();
            if self.root.projects.contains_key(&key) {
                return Ok(Some(key));
            }

            cwd = match cwd.parent() {
                Some(parent) => parent.to_path_buf(),
                None => return Ok(None),
            };
        }
    }

    fn get_project(
        &mut self,
        project_name: Option<String>,
        create_if_missing: bool
    ) -> Result<&mut Project> {
        match project_name {
            Some(name) => {
                if self.project_exists(&name) {
                    return Ok(self.get_project_from_input(&name).unwrap());
                }

                if create_if_missing {
                    return Ok(self.create_and_get_project(&name));
                }

                Err(anyhow!("Project `{}` not found", name))
            }

            None => {
                if let Some(project) = self.get_current_project()? {
                    return Ok(project);
                }

                Err(anyhow!("Not in a project directory"))
            }
        }
    }

    fn get_projects(&self) -> Vec<(String, usize)> {
        let mut project_info: Vec<(String, usize)> = vec![
            (String::from("Global"), self.root.global.tasks.len())
        ];
        project_info.extend(self
            .root
            .projects
            .iter()
            .map(|(k, p)| (self.tidy_path(k.as_str()), p.tasks.len()))
        );

        project_info
    }

    fn project_exists(&self, name: &str) -> bool {
        self.root.projects
            .keys()
            .any(|k| k.ends_with(name))
    }

    fn find_project_by_name(&mut self, project_name: &str) -> Option<&mut Project> {
        self.root.projects.iter_mut()
            .find(|(path, _)| path.ends_with(project_name))
            .map(|(_, project)| project)
    }

    fn get_global_project(&mut self) -> &mut Project {
        &mut self.root.global
    }

    fn get_current_project(&mut self) -> Result<Option<&mut Project>> {
        if let Some(k) = self.current_project_key()? {
            return Ok(self.root.projects.get_mut(&k));
        }

        Ok(None)
    }

    fn get_project_from_input(&mut self, project_name: &str) -> Option<&mut Project> {
        if project_name.eq_ignore_ascii_case("global") {
            Some(self.get_global_project())
        } else {
            self.find_project_by_name(project_name)
        }
    }

    fn create_and_get_project(&mut self, name: &str) -> &mut Project {
        self.root.projects.insert(name.to_string(), Project::default());
        self.root.projects.get_mut(name).unwrap()

    }

    fn save(&self) -> Result<()> {
        let toml_str = toml::to_string_pretty(&self.root)?;
        fs::write(self.path.join("tasks.toml"), toml_str)?;
        Ok(())
    }
}
