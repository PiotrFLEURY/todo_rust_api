use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    tags(
        (name = "Todo API", description = "API for managing todo items")
    ),
    paths(
        crate::presentation::handlers::create_todo,
        crate::presentation::handlers::get_todos,
        crate::presentation::handlers::get_todo,
        crate::presentation::handlers::update_todo,
        crate::presentation::handlers::delete_todo
    ),
    info(
        title = "Todo Rust API",
        version = "1.0.0",
        description = "An average Rust backend"
    )
)]
pub struct ApiDoc;
