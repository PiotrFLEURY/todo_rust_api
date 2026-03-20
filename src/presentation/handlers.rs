use crate::domain::models::{NewTodo, Todo, UpdateTodo};
use crate::domain::repository::TodoRepository;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::{extract::State, Json};
use logs::{debug, error, trace};

#[utoipa::path(
    post,
    path = "/todos",
    request_body = NewTodo,
    responses(
        (status = 200, description = "Todo created successfully", body = Todo),
        (status = 500, description = "Internal server error")
    )
)]
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

#[utoipa::path(
    get,
    path = "/todos",
    responses(
        (status = 200, description = "Todos fetched successfully", body = [Todo]),
        (status = 500, description = "Internal server error")
    )
)]
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

#[utoipa::path(
    get,
    path = "/todos/{id}",
    responses(
        (status = 200, description = "Todo fetched successfully", body = Todo),
        (status = 404, description = "Todo not found"),
        (status = 500, description = "Internal server error")
    )
)]
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

#[utoipa::path(
    put,
    path = "/todos/{id}",
    request_body = UpdateTodo,
    responses(
        (status = 200, description = "Todo updated successfully", body = Todo),
        (status = 404, description = "Todo not found"),
        (status = 500, description = "Internal server error")
    )
)]
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

#[utoipa::path(
    delete,
    path = "/todos/{id}",
    responses(
        (status = 204, description = "Todo deleted successfully"),
        (status = 500, description = "Internal server error")
    )
)]
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

#[cfg(test)]
mod tests {

    use crate::domain::repository::MockTodoRepository;

    use super::*;

    #[tokio::test]
    async fn test_create_todo() {
        // GIVEN
        let mut mocked_repo = MockTodoRepository::new();
        mocked_repo
            .expect_create_todo()
            .withf(|new_todo| new_todo.title == "Test Todo" && new_todo.completed == Some(false))
            .returning(|new_todo| {
                Ok(Todo {
                    id: 1,
                    title: new_todo.title.clone(),
                    completed: new_todo.completed.unwrap_or(false),
                })
            });
        let new_todo = NewTodo {
            title: "Test Todo".to_string(),
            completed: Some(false),
        };

        // WHEN
        let result = create_todo(State(mocked_repo), Json(new_todo)).await;

        // THEN
        assert!(result.is_ok());
        let Json(todo) = result.unwrap();
        assert_eq!(todo.id, 1);
        assert_eq!(todo.title, "Test Todo");
        assert_eq!(todo.completed, false);
    }

    #[tokio::test]
    async fn test_create_todo_error() {
        // GIVEN
        let mut mocked_repo = MockTodoRepository::new();
        mocked_repo
            .expect_create_todo()
            .withf(|new_todo| new_todo.title == "Test Todo" && new_todo.completed == Some(false))
            .returning(|_| {
                Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Database error",
                )))
            });
        let new_todo = NewTodo {
            title: "Test Todo".to_string(),
            completed: Some(false),
        };

        // WHEN
        let result = create_todo(State(mocked_repo), Json(new_todo)).await;

        // THEN
        assert!(result.is_err());
        let status = result.unwrap_err();
        assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_get_todos() {
        // GIVEN
        let mut mocked_repo = MockTodoRepository::new();
        mocked_repo.expect_get_todos().returning(|| {
            Ok(vec![
                Todo {
                    id: 1,
                    title: "First".to_string(),
                    completed: false,
                },
                Todo {
                    id: 2,
                    title: "Second".to_string(),
                    completed: true,
                },
            ])
        });

        // WHEN
        let result = get_todos(State(mocked_repo)).await;

        // THEN
        assert!(result.is_ok());
        let Json(todos) = result.unwrap();
        assert_eq!(todos.len(), 2);
        assert_eq!(todos[0].id, 1);
        assert_eq!(todos[1].id, 2);
    }

    #[tokio::test]
    async fn test_get_todos_error() {
        // GIVEN
        let mut mocked_repo = MockTodoRepository::new();
        mocked_repo.expect_get_todos().returning(|| {
            Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Database error",
            )))
        });

        // WHEN
        let result = get_todos(State(mocked_repo)).await;

        // THEN
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_get_todo() {
        // GIVEN
        let mut mocked_repo = MockTodoRepository::new();
        mocked_repo
            .expect_get_todo()
            .withf(|id| *id == 1)
            .returning(|id| {
                Ok(Some(Todo {
                    id: *id,
                    title: "Test Todo".to_string(),
                    completed: false,
                }))
            });

        // WHEN
        let result = get_todo(State(mocked_repo), Path(1)).await;

        // THEN
        assert!(result.is_ok());
        let Json(todo) = result.unwrap();
        assert_eq!(todo.id, 1);
        assert_eq!(todo.title, "Test Todo");
        assert!(!todo.completed);
    }

    #[tokio::test]
    async fn test_get_todo_not_found() {
        // GIVEN
        let mut mocked_repo = MockTodoRepository::new();
        mocked_repo
            .expect_get_todo()
            .withf(|id| *id == 999)
            .returning(|_| Ok(None));

        // WHEN
        let result = get_todo(State(mocked_repo), Path(999)).await;

        // THEN
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_get_todo_error() {
        // GIVEN
        let mut mocked_repo = MockTodoRepository::new();
        mocked_repo
            .expect_get_todo()
            .withf(|id| *id == 1)
            .returning(|_| {
                Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Database error",
                )))
            });

        // WHEN
        let result = get_todo(State(mocked_repo), Path(1)).await;

        // THEN
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_update_todo() {
        // GIVEN
        let mut mocked_repo = MockTodoRepository::new();
        mocked_repo
            .expect_update_todo()
            .withf(|id, payload| *id == 1 && payload.completed)
            .returning(|id, payload| {
                Ok(Some(Todo {
                    id: *id,
                    title: "Test Todo".to_string(),
                    completed: payload.completed,
                }))
            });
        let update = UpdateTodo { completed: true };

        // WHEN
        let result = update_todo(State(mocked_repo), Path(1), Json(update)).await;

        // THEN
        assert!(result.is_ok());
        let Json(todo) = result.unwrap();
        assert_eq!(todo.id, 1);
        assert!(todo.completed);
    }

    #[tokio::test]
    async fn test_update_todo_not_found() {
        // GIVEN
        let mut mocked_repo = MockTodoRepository::new();
        mocked_repo
            .expect_update_todo()
            .withf(|id, payload| *id == 999 && payload.completed)
            .returning(|_, _| Ok(None));
        let update = UpdateTodo { completed: true };

        // WHEN
        let result = update_todo(State(mocked_repo), Path(999), Json(update)).await;

        // THEN
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_update_todo_error() {
        // GIVEN
        let mut mocked_repo = MockTodoRepository::new();
        mocked_repo
            .expect_update_todo()
            .withf(|id, payload| *id == 1 && payload.completed)
            .returning(|_, _| {
                Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Database error",
                )))
            });
        let update = UpdateTodo { completed: true };

        // WHEN
        let result = update_todo(State(mocked_repo), Path(1), Json(update)).await;

        // THEN
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_delete_todo() {
        // GIVEN
        let mut mocked_repo = MockTodoRepository::new();
        mocked_repo
            .expect_delete_todo()
            .withf(|id| *id == 1)
            .returning(|_| Ok(()));

        // WHEN
        let result = delete_todo(State(mocked_repo), Path(1)).await;

        // THEN
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), StatusCode::NO_CONTENT);
    }

    #[tokio::test]
    async fn test_delete_todo_error() {
        // GIVEN
        let mut mocked_repo = MockTodoRepository::new();
        mocked_repo
            .expect_delete_todo()
            .withf(|id| *id == 1)
            .returning(|_| {
                Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Database error",
                )))
            });

        // WHEN
        let result = delete_todo(State(mocked_repo), Path(1)).await;

        // THEN
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), StatusCode::INTERNAL_SERVER_ERROR);
    }
}
