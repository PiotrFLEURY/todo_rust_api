use std::error::Error;

use axum::async_trait;

use crate::domain::models::{NewTodo, Todo, UpdateTodo};

#[async_trait]
pub trait TodoRepository: Send + Sync {
    ///
    /// Creates a new todo item
    ///
    async fn create_todo(&self, new_todo: &NewTodo) -> Result<Todo, Box<dyn Error>>;

    ///
    /// Retrieves all todo items
    ///
    async fn get_todos(&self) -> Result<Vec<Todo>, Box<dyn Error>>;

    ///
    /// Retrieves a single todo item by its ID
    ///
    async fn get_todo(&self, id: &i32) -> Result<Option<Todo>, Box<dyn Error>>;

    ///
    /// Updates an existing todo item by its ID
    ///
    async fn update_todo(
        &self,
        id: &i32,
        update_todo: &UpdateTodo,
    ) -> Result<Option<Todo>, Box<dyn Error>>;

    ///
    /// Deletes a todo item by its ID
    ///
    async fn delete_todo(&self, id: &i32) -> Result<(), Box<dyn Error>>;
}
