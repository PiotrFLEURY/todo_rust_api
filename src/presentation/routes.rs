use crate::domain::repository::TodoRepository;
use crate::presentation::api::ApiDoc;
use crate::presentation::handlers;
use axum::routing::{delete, get, post, put};
use axum::Router;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

///
/// Creates the router for the application, defining the routes and their corresponding handlers.
///
/// Generic type R is used to allow for different implementations of the TodoRepository, enabling flexibility and testability.
/// The notation <R: TodoRepository + Clone + 'static> ensures that R must implement the TodoRepository trait, be cloneable, and have a static lifetime.
/// This allows the router to work with any repository implementation that meets these criteria, making it easier to swap out the data layer without changing the routing logic.
/// The with_state method is used to pass the repository instance to the handlers, allowing them to access the data layer when processing requests.
/// This design promotes separation of concerns, as the routing logic is decoupled from the data access logic, and
/// allows for better modularity and maintainability of the codebase.
///
pub fn create_router<R: TodoRepository + Clone + 'static>(repo: R) -> Router {
    Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .route("/todos", post(handlers::create_todo::<R>))
        .route("/todos", get(handlers::get_todos::<R>))
        .route("/todos/{id}", get(handlers::get_todo::<R>))
        .route("/todos/{id}", put(handlers::update_todo::<R>))
        .route("/todos/{id}", delete(handlers::delete_todo::<R>))
        .with_state(repo)
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use async_trait::async_trait;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use tower::ServiceExt;

    use crate::domain::{models::Todo, repository::TodoRepository};

    use super::*;

    #[derive(Clone)]
    struct TestRepo;

    #[async_trait]
    impl TodoRepository for TestRepo {
        async fn create_todo(
            &self,
            new_todo: &crate::domain::models::NewTodo,
        ) -> Result<Todo, Box<dyn Error>> {
            Ok(Todo {
                id: 1,
                title: new_todo.title.clone(),
                completed: new_todo.completed.unwrap_or(false),
            })
        }

        async fn get_todos(&self) -> Result<Vec<Todo>, Box<dyn Error>> {
            Ok(vec![])
        }

        async fn get_todo(&self, id: &i32) -> Result<Option<Todo>, Box<dyn Error>> {
            Ok(Some(Todo {
                id: *id,
                title: "Test Todo".to_string(),
                completed: false,
            }))
        }

        async fn update_todo(
            &self,
            id: &i32,
            update_todo: &crate::domain::models::UpdateTodo,
        ) -> Result<Option<Todo>, Box<dyn Error>> {
            Ok(Some(Todo {
                id: *id,
                title: "Test Todo".to_string(),
                completed: update_todo.completed,
            }))
        }

        async fn delete_todo(&self, _id: &i32) -> Result<(), Box<dyn Error>> {
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_post_todos_route() {
        // GIVEN
        let app = create_router(TestRepo);

        let payload = r#"{"title":"Test Todo","completed":false}"#;

        let request = Request::builder()
            .method("POST")
            .uri("/todos")
            .header("content-type", "application/json")
            .body(Body::from(payload.to_string()))
            .unwrap();

        // WHEN
        let response = app.oneshot(request).await.unwrap();

        // THEN
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_get_todos_route() {
        // GIVEN
        let app = create_router(TestRepo);

        let request = Request::builder()
            .method("GET")
            .uri("/todos")
            .body(Body::empty())
            .unwrap();

        // WHEN
        let response = app.oneshot(request).await.unwrap();

        // THEN
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_get_todo_route() {
        // GIVEN
        let app = create_router(TestRepo);

        let request = Request::builder()
            .method("GET")
            .uri("/todos/1")
            .body(Body::empty())
            .unwrap();

        // WHEN
        let response = app.oneshot(request).await.unwrap();

        // THEN
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_put_todo_route() {
        // GIVEN
        let app = create_router(TestRepo);

        let payload = r#"{"completed":true}"#;
        let request = Request::builder()
            .method("PUT")
            .uri("/todos/1")
            .header("content-type", "application/json")
            .body(Body::from(payload.to_string()))
            .unwrap();

        // WHEN
        let response = app.oneshot(request).await.unwrap();

        // THEN
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_delete_todo_route() {
        // GIVEN
        let app = create_router(TestRepo);

        let request = Request::builder()
            .method("DELETE")
            .uri("/todos/1")
            .body(Body::empty())
            .unwrap();

        // WHEN
        let response = app.oneshot(request).await.unwrap();

        // THEN
        assert_eq!(response.status(), StatusCode::NO_CONTENT);
    }
}
