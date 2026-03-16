use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub completed: bool,
}

#[derive(Debug, Deserialize)]
pub struct NewTodo {
    pub title: String,
    pub completed: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTodo {
    pub completed: bool,
}
