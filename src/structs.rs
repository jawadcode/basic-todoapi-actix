use serde::{Deserialize, Serialize};
use std::sync::RwLock;

pub type Todos = Vec<Todo>;

pub struct Store {
    pub todos: RwLock<Todos>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Todo {
    pub title: String,
    pub description: String,
    pub completed: bool,
}

impl Store {
    pub fn new() -> Self {
        Self {
            todos: RwLock::new(Vec::new()),
        }
    }
}

#[derive(Deserialize)]
pub struct QueryParams {
    pub query: Option<String>,
    pub limit: Option<usize>,
    pub filter: Option<String>,
}