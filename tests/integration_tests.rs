use axum::{
    body::Body,
    http::{Request, StatusCode},
};

use tower::ServiceExt;

mod common;

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
