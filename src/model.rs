use serde::{Deserialize, Serialize};

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
