use axum::{
    body::{to_bytes, Body},
    http::{Request, StatusCode},
};

use todo_rust_api::domain::models::Todo;
use tower::ServiceExt;

mod common;

#[tokio::test]
async fn test_swagger_ui() {
    let test_context = common::setup().await.expect("Unable to setup test context");
    let app = test_context.app;
    let container = test_context.container;

    // WHEN
    let request = Request::builder()
        .method("GET")
        .uri("/swagger-ui/index.html")
        .body(Body::empty())
        .unwrap();
    let response = app.oneshot(request).await.unwrap();

    // THEN
    assert_eq!(response.status(), StatusCode::OK);

    common::teardown(container).await;
}

#[tokio::test]
async fn test_create_todo() {
    let test_context = common::setup().await.expect("Unable to setup test context");
    let app = test_context.app;
    let container = test_context.container;

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

    common::teardown(container).await;
}

#[tokio::test]
async fn test_update_todo() {
    let test_context = common::setup().await.expect("Unable to setup test context");
    let app = test_context.app;
    let container = test_context.container;

    let payload = r#"{"title":"Test Todo","completed":false}"#;

    let create_request = Request::builder()
        .method("POST")
        .uri("/todos")
        .header("content-type", "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();
    let create_response = app.clone().oneshot(create_request).await.unwrap();
    assert_eq!(create_response.status(), StatusCode::OK);

    // WHEN
    let update_payload = r#"{"title":"Test Todo","completed":true}"#;
    let update_request = Request::builder()
        .method("PUT")
        .uri("/todos/1")
        .header("content-type", "application/json")
        .body(Body::from(update_payload.to_string()))
        .unwrap();
    let update_response = app.clone().oneshot(update_request).await.unwrap();

    // THEN
    assert_eq!(update_response.status(), StatusCode::OK);
    let body = to_bytes(update_response.into_body(), usize::MAX)
        .await
        .expect("Failed to read response body");
    let updated_todo: Todo = serde_json::from_slice(&body).expect("Failed to parse response body");
    assert_eq!(updated_todo.title, "Test Todo");
    assert_eq!(updated_todo.completed, true);

    common::teardown(container).await;
}

#[tokio::test]
async fn test_get_todos() {
    let test_context = common::setup().await.expect("Unable to setup test context");
    let app = test_context.app;
    let container = test_context.container;

    let payload = r#"{"title":"Test Todo","completed":false}"#;

    for _ in 0..5 {
        let create_request = Request::builder()
            .method("POST")
            .uri("/todos")
            .header("content-type", "application/json")
            .body(Body::from(payload.to_string()))
            .unwrap();
        let response = app.clone().oneshot(create_request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    // WHEN
    let get_request = Request::builder()
        .method("GET")
        .uri("/todos")
        .body(Body::empty())
        .unwrap();
    let response = app.oneshot(get_request).await.unwrap();

    // THEN
    assert_eq!(response.status(), StatusCode::OK);
    let body = to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("Failed to read response body");
    let todo_list: Vec<Todo> =
        serde_json::from_slice(&body).expect("Failed to parse response body");
    assert_eq!(todo_list.len(), 5);
    for todo in todo_list {
        assert_eq!(todo.title, "Test Todo");
        assert_eq!(todo.completed, false);
    }

    common::teardown(container).await;
}

#[tokio::test]
async fn test_get_one_todo() {
    let test_context = common::setup().await.expect("Unable to setup test context");
    let app = test_context.app;
    let container = test_context.container;

    let payload = r#"{"title":"Test Todo","completed":false}"#;

    for _ in 0..5 {
        let create_request = Request::builder()
            .method("POST")
            .uri("/todos")
            .header("content-type", "application/json")
            .body(Body::from(payload.to_string()))
            .unwrap();
        let response = app.clone().oneshot(create_request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    // WHEN
    let get_request = Request::builder()
        .method("GET")
        .uri("/todos/3")
        .body(Body::empty())
        .unwrap();
    let response = app.oneshot(get_request).await.unwrap();

    // THEN
    assert_eq!(response.status(), StatusCode::OK);
    let body = to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("Failed to read response body");
    let todo: Todo = serde_json::from_slice(&body).expect("Failed to parse response body");
    assert_eq!(todo.id, 3);
    assert_eq!(todo.title, "Test Todo");
    assert_eq!(todo.completed, false);

    common::teardown(container).await;
}

#[tokio::test]
async fn test_delete_one_todo() {
    let test_context = common::setup().await.expect("Unable to setup test context");
    let app = test_context.app;
    let container = test_context.container;

    let payload = r#"{"title":"Test Todo","completed":false}"#;

    for _ in 0..5 {
        let create_request = Request::builder()
            .method("POST")
            .uri("/todos")
            .header("content-type", "application/json")
            .body(Body::from(payload.to_string()))
            .unwrap();
        let response = app.clone().oneshot(create_request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    // WHEN
    let delete_request = Request::builder()
        .method("DELETE")
        .uri("/todos/3")
        .body(Body::empty())
        .unwrap();
    let response = app.clone().oneshot(delete_request).await.unwrap();

    // THEN
    assert_eq!(response.status(), StatusCode::NO_CONTENT);

    let get_request = Request::builder()
        .method("GET")
        .uri("/todos/3")
        .body(Body::empty())
        .unwrap();
    let response = app.clone().oneshot(get_request).await.unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);

    let get_all_request = Request::builder()
        .method("GET")
        .uri("/todos")
        .body(Body::empty())
        .unwrap();
    let response = app.clone().oneshot(get_all_request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let body = to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("Failed to read response body");
    let todo_list: Vec<Todo> =
        serde_json::from_slice(&body).expect("Failed to parse response body");
    assert_eq!(todo_list.len(), 4);
    for todo in todo_list {
        assert_ne!(todo.id, 3);
        assert_eq!(todo.title, "Test Todo");
        assert_eq!(todo.completed, false);
    }

    common::teardown(container).await;
}
