use crate::domain::models::{NewTodo, Todo, UpdateTodo};
use crate::domain::repository::TodoRepository;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::{extract::State, Json};
use logs::{debug, error, trace};

pub async fn create_todo<R: TodoRepository>(
    State(repo): State<R>,
    Json(new_todo): Json<NewTodo>,
) -> Result<Json<Todo>, StatusCode> {
    trace!("Creating new todo: {:?}", new_todo);
    let todo = repo.create_todo(&new_todo).await.map_err(|e| {
        error!("Failed to create todo: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    debug!("Created todo: {:?}", todo);
    Ok(Json(todo))
}

pub async fn get_todos<R: TodoRepository>(
    State(repo): State<R>,
) -> Result<Json<Vec<Todo>>, StatusCode> {
    trace!("Fetching all todos");
    let todos = repo.get_todos().await.map_err(|e| {
        error!("Failed to fetch todos: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    debug!("Fetched todos: {:?}", todos);
    Ok(Json(todos))
}

pub async fn get_todo<R: TodoRepository>(
    State(repo): State<R>,
    Path(id): Path<i32>,
) -> Result<Json<Todo>, StatusCode> {
    trace!("Fetching todo with id: {}", id);
    let todo = repo.get_todo(&id).await.map_err(|e| {
        error!("Failed to fetch todo with id {}: {}", id, e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    match todo {
        Some(todo) => {
            debug!("Fetched todo: {:?}", todo);
            Ok(Json(todo))
        }
        None => {
            error!("Todo with id {} not found", id);
            Err(StatusCode::NOT_FOUND)
        }
    }
}

pub async fn update_todo<R: TodoRepository>(
    State(repo): State<R>,
    Path(id): Path<i32>,
    Json(update_todo): Json<UpdateTodo>,
) -> Result<Json<Todo>, StatusCode> {
    trace!("Updating todo with id: {} with data: {:?}", id, update_todo);
    let todo = repo.update_todo(&id, &update_todo).await.map_err(|e| {
        error!("Failed to update todo with id {}: {}", id, e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    match todo {
        Some(todo) => {
            debug!("Updated todo: {:?}", todo);
            Ok(Json(todo))
        }
        None => {
            error!("Todo with id {} not found", id);
            Err(StatusCode::NOT_FOUND)
        }
    }
}

pub async fn delete_todo<R: TodoRepository>(
    State(repo): State<R>,
    Path(id): Path<i32>,
) -> Result<StatusCode, StatusCode> {
    trace!("Deleting todo with id: {}", id);
    repo.delete_todo(&id).await.map_err(|e| {
        error!("Failed to delete todo with id {}: {}", id, e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    debug!("Deleted todo with id: {}", id);
    Ok(StatusCode::NO_CONTENT)
}
