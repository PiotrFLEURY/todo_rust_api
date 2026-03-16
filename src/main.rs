use dotenv::dotenv;
use std::env;

use crate::{
    data::{db::create_pool, repository::TodoRepositoryImpl},
    presentation::routes::create_router,
};

mod data;
mod domain;
mod presentation;

#[tokio::main]
async fn main() {
    // Loading environment variables from .env file
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let bind_address = env::var("BIND_ADDRESS").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());

    let pool = create_pool(&database_url).await;
    let repo = TodoRepositoryImpl::new(pool);
    let router = create_router(repo);

    let server_address = format!("{}:{}", bind_address, port);

    let listener = tokio::net::TcpListener::bind(&server_address)
        .await
        .expect(format!("Unable to bind address {} on port {}", &bind_address, &port).as_str());
    axum::serve(listener, router)
        .await
        .expect("Failed to start server");
}
