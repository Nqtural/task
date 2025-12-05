use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};

// TODO:
// - privatize all fields
// - getter and setter functions
// - default implementation for TaskList

#[derive(Deserialize, Serialize, Debug)]
pub struct Task {
    pub id: u16,
    pub name: String,
    pub finished: bool,
    pub expiration: Option<i64>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TaskList {
    pub task: Vec<Task>,
}

impl TaskList {
    pub fn new() -> Self {
        Self {
            task: Vec::new(),
        }
    }

    pub fn get_task_by_id(&mut self, id: u16) -> Result<&mut Task> {
        self.task.iter_mut()
            .find(|t| t.id == id)
            .ok_or_else(|| anyhow!("Task with id {} not found", id))
    }
}
