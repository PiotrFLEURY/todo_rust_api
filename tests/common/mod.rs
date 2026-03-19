use std::time::{Duration, Instant};

use axum::Router;
use logs::Logs;
use testcontainers::{
    core::{ExecCommand, IntoContainerPort, WaitFor},
    runners::AsyncRunner,
    ContainerAsync, GenericImage, ImageExt,
};
use todo_rust_api::{
    data::{db, repository::TodoRepositoryImpl},
    presentation::routes::create_router,
};

pub struct TestContext {
    pub app: Router,
    pub container: ContainerAsync<GenericImage>,
}

impl TestContext {
    pub fn new(app: Router, container: ContainerAsync<GenericImage>) -> Self {
        Self { app, container }
    }
}

///
/// Utils to setup logs when coding tests.
///
/// /!\ Do not init logs in every tests. It will make tests panic because of multiple initialization.
/// You can init logs in a single test and it will work for all tests because logs are global.
///
#[allow(dead_code)]
pub fn init_logs() {
    Logs::new()
        // Show log level color
        .color(true)
        // Filter log level
        .level(logs::LevelFilter::Trace)
        .init();
}

pub async fn setup() -> Result<TestContext, Box<dyn std::error::Error>> {
    // Create postgres container
    let container_port = 5432.tcp();
    let container = GenericImage::new("postgres", "16")
        .with_exposed_port(container_port)
        .with_wait_for(WaitFor::message_on_stdout(
            "PostgreSQL init process complete; ready for start up",
        ))
        .with_env_var("POSTGRES_PASSWORD", "postgres")
        .with_startup_timeout(Duration::from_secs(10))
        .start()
        .await
        .expect("Failed to start Postgres container");

    // Resolve the host and dynamically mapped port for the Postgres container
    let host = container.get_host().await?;
    let host_port = container.get_host_port_ipv4(container_port).await?;

    let database_url = format!(
        "postgres://postgres:postgres@{}:{}/postgres",
        host, host_port
    );

    // Wait for the database to be ready by retrying connections with timeout/backoff
    let start = Instant::now();
    let timeout = Duration::from_secs(30);
    loop {
        match db::connect_to_db(&database_url, false).await {
            Ok(_) => break,
            Err(err) => {
                if start.elapsed() >= timeout {
                    panic!("Timed out waiting for Postgres to be ready: {err}");
                }
                tokio::time::sleep(Duration::from_millis(500)).await;
            }
        }
    }

    // Create table
    container
        .exec(ExecCommand::new(
            vec![
                "psql",
                "-U",
                "postgres",
                "-c",
                "CREATE TABLE IF NOT EXISTS todos (id SERIAL PRIMARY KEY, title TEXT NOT NULL, completed BOOLEAN NOT NULL);",
            ]
        ))
             .await
        .expect("Failed to create table");

    let db_connection = db::connect_to_db(&database_url, true)
        .await
        .expect("Failed to connect to database");

    let todo_repository = TodoRepositoryImpl::new(db_connection);

    let router = create_router(todo_repository);

    Ok(TestContext::new(router, container))
}

pub async fn teardown(container: ContainerAsync<GenericImage>) {
    container.stop().await.expect("Failed to stop container");
}
