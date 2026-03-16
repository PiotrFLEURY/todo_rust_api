use std::error::Error;

use axum::async_trait;
use sqlx::PgPool;

use crate::domain::{
    models::{NewTodo, Todo, UpdateTodo},
    repository::TodoRepository,
};

#[derive(Debug, Clone)]
pub struct TodoRepositoryImpl {
    pool: PgPool,
}

impl TodoRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TodoRepository for TodoRepositoryImpl {
    async fn create_todo(&self, new_todo: &NewTodo) -> Result<Todo, Box<dyn Error>> {
        let completed = new_todo.completed.unwrap_or(false);
        sqlx::query_as!(
            Todo,
            r#"
        INSERT INTO todos (title, completed)
        VALUES ($1, $2)
        RETURNING id, title, completed
        "#,
            new_todo.title,
            completed
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e: sqlx::Error| e.into())
    }

    async fn get_todos(&self) -> Result<Vec<Todo>, Box<dyn Error>> {
        sqlx::query_as!(Todo, r#"SELECT id, title, completed FROM todos"#)
            .fetch_all(&self.pool)
            .await
            .map_err(|e: sqlx::Error| e.into())
    }

    async fn get_todo(&self, id: &i32) -> Result<Option<Todo>, Box<dyn Error>> {
        sqlx::query_as!(
            Todo,
            r#"SELECT id, title, completed FROM todos WHERE id = $1"#,
            id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e: sqlx::Error| e.into())
    }

    async fn update_todo(
        &self,
        id: &i32,
        update_todo: &UpdateTodo,
    ) -> Result<Option<Todo>, Box<dyn Error>> {
        sqlx::query_as!(
            Todo,
            r#"
        UPDATE todos
        SET completed = $1
        WHERE id = $2
        RETURNING id, title, completed
        "#,
            update_todo.completed,
            id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e: sqlx::Error| e.into())
    }

    async fn delete_todo(&self, id: &i32) -> Result<(), Box<dyn Error>> {
        sqlx::query!(
            r#"
        DELETE FROM todos
        WHERE id = $1
        "#,
            id
        )
        .execute(&self.pool)
        .await
        .map_err(|e: sqlx::Error| <sqlx::Error as Into<Box<dyn Error>>>::into(e))?;

        Ok(())
    }
}
