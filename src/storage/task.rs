use anyhow::Result;
use rusqlite::params;

use crate::types::Task;
use super::TaskStorage;

impl TaskStorage {
    pub fn add_task(
        &self,
        project_id: u32,
        name: &str,
        expiration: Option<&str>,
    ) -> Result<()> {
        self.conn.execute(
            "INSERT INTO tasks (project_id, name, finished, expiration)
            VALUES (?1, ?2, ?3, ?4)",
            params![project_id, name, 0, expiration],
        )?;

        Ok(())
    }

    pub fn delete_task(&self, task_id: u32) -> Result<()> {
        self.conn.execute(
            "DELETE FROM tasks
            WHERE id = ?1",
            params![task_id],
        )?;

        Ok(())
    }

    pub fn update_task(
        &self,
        task_id: u32,
        name: Option<&str>,
        expiration: Option<i64>,
    ) -> Result<()> {
        self.conn.execute(
            "UPDATE tasks
             SET name = COALESCE(?1, name), expiration = COALESCE(?2, expiration)
             WHERE id = ?3",
            params![name, expiration, task_id],
        )?;

        Ok(())
    }

    pub fn toggle_finish_task(&self, task_id: u32) -> Result<()> {
        self.conn.execute(
            "UPDATE tasks
            SET finished = 1 - finished
            WHERE id = ?1",
            params![task_id],
        )?;

        Ok(())
    }

    pub fn get_task(&self, task_id: u32) -> Result<Task> {
        Ok(self.conn.prepare(
            "SELECT * FROM tasks
            WHERE id = ?1",
        )?
            .query_row([task_id], |row| {
                Ok(Task {
                    id: row.get(0)?,
                    project_id: row.get(1)?,
                    name: row.get(2)?,
                    finished: row.get(3)?,
                    expiration: row.get(4)?,
                })
            })?
        )
    }

    pub fn get_tasks(&self, project_id: u32) -> Result<Vec<Task>> {
        Ok(self.conn.prepare(
            "SELECT * FROM tasks
            WHERE project_id = ?1
            ORDER BY id"
        )?
            .query_map([project_id], |row| {
                Ok(Task {
                    id: row.get(0)?,
                    project_id: row.get(1)?,
                    name: row.get(2)?,
                    finished: row.get(3)?,
                    expiration: row.get(4)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?)
    }
}
