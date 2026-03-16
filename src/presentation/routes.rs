use crate::domain::repository::TodoRepository;
use crate::presentation::handlers;
use axum::routing::{delete, get, post, put};
use axum::Router;

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
        .route("/todos", post(handlers::create_todo::<R>))
        .route("/todos", get(handlers::get_todos::<R>))
        .route("/todos/:id", get(handlers::get_todo::<R>))
        .route("/todos/:id", put(handlers::update_todo::<R>))
        .route("/todos/:id", delete(handlers::delete_todo::<R>))
        .with_state(repo)
}
