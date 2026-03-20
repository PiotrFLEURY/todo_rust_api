use std::error::Error;

use async_trait::async_trait;
use sea_orm::ActiveModelTrait;
use sea_orm::ActiveValue::NotSet;
use sea_orm::ActiveValue::Set;
use sea_orm::DatabaseConnection;
use sea_orm::EntityTrait;

use crate::domain::{
    models::{NewTodo, Todo, UpdateTodo},
    repository::TodoRepository,
};

use crate::data::dao::ActiveModel as TodoActiveModel;
use crate::data::dao::Entity as TodoDao;

#[derive(Debug, Clone)]
pub struct TodoRepositoryImpl {
    db: DatabaseConnection,
}

impl TodoRepositoryImpl {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

#[async_trait]
impl TodoRepository for TodoRepositoryImpl {
    async fn create_todo(&self, new_todo: &NewTodo) -> Result<Todo, Box<dyn Error>> {
        let todo = TodoActiveModel {
            id: NotSet, // will be ignored by the database
            title: Set(new_todo.title.clone()),
            completed: Set(new_todo.completed.unwrap_or(false)),
        }
        .insert(&self.db)
        .await?;

        Ok(todo.to_domain())
    }

    async fn get_todos(&self) -> Result<Vec<Todo>, Box<dyn Error>> {
        let rows = TodoDao::find().all(&self.db).await?;

        let todos = rows.into_iter().map(|model| model.to_domain()).collect();

        Ok(todos)
    }

    async fn get_todo(&self, id: &i32) -> Result<Option<Todo>, Box<dyn Error>> {
        let row = TodoDao::find_by_id(*id).one(&self.db).await?;

        Ok(row.map(|model| model.to_domain()))
    }

    async fn update_todo(
        &self,
        id: &i32,
        update_todo: &UpdateTodo,
    ) -> Result<Option<Todo>, Box<dyn Error>> {
        let row = TodoDao::find_by_id(*id).one(&self.db).await?;

        // Into ActiveModel
        let mut todo: TodoActiveModel = match row {
            Some(model) => model.into(),
            None => return Ok(None),
        };

        // Update fields
        todo.completed = Set(update_todo.completed);

        // Update in database
        let updated_todo = todo.update(&self.db).await?;

        Ok(Some(updated_todo.to_domain()))
    }

    async fn delete_todo(&self, id: &i32) -> Result<(), Box<dyn Error>> {
        TodoDao::delete_by_id(*id).exec(&self.db).await?;

        Ok(())
    }
}
