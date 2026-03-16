use crate::domain::models::{NewTodo, Todo, UpdateTodo};
use crate::domain::repository::TodoRepository;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::{extract::State, Json};

pub async fn create_todo<R: TodoRepository>(
    State(repo): State<R>,
    Json(new_todo): Json<NewTodo>,
) -> Result<Json<Todo>, StatusCode> {
    let todo = repo
        .create_todo(&new_todo)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(todo))
}

pub async fn get_todos<R: TodoRepository>(
    State(repo): State<R>,
) -> Result<Json<Vec<Todo>>, StatusCode> {
    let todos = repo
        .get_todos()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(todos))
}

pub async fn get_todo<R: TodoRepository>(
    State(repo): State<R>,
    Path(id): Path<i32>,
) -> Result<Json<Todo>, StatusCode> {
    let todo = repo
        .get_todo(&id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    match todo {
        Some(todo) => Ok(Json(todo)),
        None => Err(StatusCode::NOT_FOUND),
    }
}

pub async fn update_todo<R: TodoRepository>(
    State(repo): State<R>,
    Path(id): Path<i32>,
    Json(update_todo): Json<UpdateTodo>,
) -> Result<Json<Todo>, StatusCode> {
    let todo = repo
        .update_todo(&id, &update_todo)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    match todo {
        Some(todo) => Ok(Json(todo)),
        None => Err(StatusCode::NOT_FOUND),
    }
}

pub async fn delete_todo<R: TodoRepository>(
    State(repo): State<R>,
    Path(id): Path<i32>,
) -> Result<StatusCode, StatusCode> {
    repo.delete_todo(&id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(StatusCode::NO_CONTENT)
}
