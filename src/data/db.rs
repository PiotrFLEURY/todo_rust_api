use std::time::Duration;

use sea_orm::{ConnectOptions, ConnectionTrait, Database, DatabaseConnection, Statement};

pub async fn update_db(connection: &DatabaseConnection) -> Result<(), Box<dyn std::error::Error>> {
    // List files into db directory
    let entries = std::fs::read_dir("db")?
        .filter_map(|res| res.ok())
        .filter(|entry| entry.path().is_file())
        .filter(|entry| entry.path().extension().map_or(false, |ext| ext == "sql"))
        .collect::<Vec<_>>();

    let queries = entries
        .iter()
        .map(|entry| std::fs::read_to_string(entry.path()).ok())
        .filter_map(|res| res)
        .collect::<Vec<_>>();

    for query in queries {
        connection
            .execute_raw(Statement::from_string(
                connection.get_database_backend(),
                query,
            ))
            .await?;
    }
    Ok(())
}

pub async fn connect_to_db(
    database_url: &str,
    logging: bool,
) -> Result<DatabaseConnection, Box<dyn std::error::Error>> {
    let mut opt = ConnectOptions::new(database_url.to_string());
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .sqlx_logging(logging);

    let connection = Database::connect(opt).await?;
    Ok(connection)
}
