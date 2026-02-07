use anyhow::Result;
use rusqlite::{OptionalExtension, params};

use crate::types::Project;
use super::TaskStorage;

impl TaskStorage {
    pub fn new_project(&self) -> Result<()> {
        self.conn.execute(
            "INSERT INTO projects (path)
            VALUES (?1)",
            params![get_cwd()?],
        )?;
        Ok(())
    }

    pub fn delete_project(&self, id: u32) -> Result<()> {
        self.conn.execute(
            "DELETE FROM projects
            WHERE id = ?1",
            params![id],
        )?;
        Ok(())
    }

    pub fn get_current_project(&self) -> Result<Option<u32>> {
        self.find_project_by_dir_name(&get_cwd()?)
    }

    pub fn get_project(&self, id: u32) -> Result<Option<Project>> {
        let project = self.conn.prepare(
            "SELECT * FROM projects
            WHERE id = ?1"
        )?
            .query_row([id], |row| {
                Ok(Project {
                    id: row.get(0)?,
                    path: row.get(1)?,
                    tasks: Vec::new(),
                })
            })
            .optional()?;

        if let Some(mut project) = project {
            project.tasks = self.get_tasks(project.id)?;

            Ok(Some(project))
        } else {
            Ok(None)
        }
    }

    pub fn get_all_projects(&self) -> Result<Vec<Project>> {
        let mut projects = self.conn.prepare(
            "SELECT id, path
            FROM projects
            ORDER BY id"
        )?
            .query_map([], |row| {
                Ok(Project {
                    id: row.get(0)?,
                    path: row.get(1)?,
                    tasks: Vec::new(),
                })
            })?
        .collect::<Result<Vec<_>, _>>()?;

        for project in &mut projects {
            project.tasks = self.get_tasks(project.id)?;
        }

        Ok(projects)
    }

    pub fn find_project_by_dir_name(&self, name: &str) -> Result<Option<u32>> {
        Ok(self.conn
            .query_row(
                "SELECT id FROM projects
                WHERE path = ?1 OR path LIKE ?2
                ORDER BY LENGTH(path) DESC
                LIMIT 1",
                [name, &format!("%/{}", name)],
                |row| row.get(0),
            )
            .optional()?)
    }
}

fn get_cwd() -> Result<String> {
    Ok(std::env::current_dir()?.to_string_lossy().to_string())
}
