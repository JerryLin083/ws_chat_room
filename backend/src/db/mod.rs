use sqlx::{
    Pool, Postgres,
    postgres::{PgConnectOptions, PgPoolOptions},
};
use std::{str::FromStr, time::Duration};
use tokio::sync::mpsc::{self, Sender};
use uuid::Uuid;

use crate::room_manager::RoomCommand;

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

pub async fn db_messages_connection() -> Sender<RoomCommand> {
    let (sender, mut receiver) = mpsc::channel::<RoomCommand>(128);

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

    tokio::spawn(async move {
        while let Some(command) = receiver.recv().await {
            let query_str = r#"
                insert into messages(room_id, user_id, content)
                values($1, $2, $3)
            "#;

            if let Ok(room_id) = Uuid::from_str(&command.room_id.unwrap()) {
                let _ = sqlx::query(query_str)
                    .bind(room_id)
                    .bind(&command.user_id.unwrap())
                    .bind(&command.message.unwrap())
                    .execute(&pool)
                    .await
                    .map_err(|err| {
                        tracing::error!("Failed to insert message: {}", err.to_string())
                    });
            };
        }
    });

    tracing::info!("Listening on message...");

    sender
}
