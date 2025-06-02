use std::time::Duration;

use sqlx::{
    Pool, Postgres,
    postgres::{PgConnectOptions, PgPoolOptions},
};

pub async fn db_connection() -> Pool<Postgres> {
    let connection_option = PgConnectOptions::new()
        .host(&dotenv::var("HOST").unwrap())
        .username(&dotenv::var("USER").unwrap())
        .password(&dotenv::var("PASSWORD").unwrap())
        .database(&dotenv::var("DBNAME").unwrap())
        .ssl_mode(sqlx::postgres::PgSslMode::Require);

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect_with(connection_option)
        .await
        .map_err(|err| panic!("Database connection error: {}", err))
        .unwrap();

    tracing::info!("Database connected...");

    pool
}
