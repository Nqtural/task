use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub id: u32,
    pub name: String,
    pub finished: bool,
    pub expiration: Option<i64>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Project {
    pub tasks: Vec<Task>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Global {
    pub tasks: Vec<Task>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Root {
    pub global: Global,
    pub projects: HashMap<String, Project>,
}
