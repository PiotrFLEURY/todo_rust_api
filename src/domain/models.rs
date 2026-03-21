use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub completed: bool,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct NewTodo {
    pub title: String,
    pub completed: Option<bool>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateTodo {
    pub completed: bool,
}
