use axum::{Router, http::StatusCode, routing::get};
use sqlx::{Pool, Postgres};

pub async fn router(pool: Pool<Postgres>) -> Router {
    let app = Router::new().route("/", get(greeting)).with_state(pool);

    tracing::info!("Router init...");

    app
}

async fn greeting() -> Result<String, (StatusCode, String)> {
    Ok(String::from("Hello world"))
}
