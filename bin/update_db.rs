use std::env;

use dotenv::dotenv;
use logs::{debug, Logs};
use todo_rust_api::data::db;

#[tokio::main]
async fn main() {
    dotenv().ok();

    Logs::new()
        // Show log level color
        .color(true)
        // Filter log level
        .level_from_env("LOG_LEVEL")
        .unwrap_or_default()
        .init();

    debug!("Loading environment variables from .env file");
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    debug!("Got environment variable DATABASE_URL={}", database_url);

    debug!("Creating database connection pool");
    let db_connection = db::connect_to_db(&database_url, false)
        .await
        .expect("Failed to connect to database");

    debug!("Updating database schema");
    db::update_db(&db_connection)
        .await
        .expect("Failed to update database schema");

    debug!("Database schema updated successfully");
}
