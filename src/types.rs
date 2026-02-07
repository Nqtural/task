use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub id: u32,
    pub project_id: u32,
    pub name: String,
    pub finished: bool,
    pub expiration: Option<i64>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Project {
    pub id: u32,
    pub path: String,
    pub tasks: Vec<Task>,
}
