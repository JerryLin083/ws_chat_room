use std::sync::Arc;

use axum::{Router, extract::State, http::StatusCode, routing::get};
use sqlx::{Pool, Postgres, Row};

use crate::session::SessionManager;

pub async fn router(pool: Pool<Postgres>, session_manager: Arc<SessionManager>) -> Router {
    let app_state = AppState {
        pool,
        session_manager,
    };

    let app = Router::new()
        .route("/", get(greeting))
        .with_state(Arc::new(app_state));

    tracing::info!("Router init...");

    app
}

pub struct AppState {
    pool: Pool<Postgres>,
    session_manager: Arc<SessionManager>,
}

async fn greeting(State(app_state): State<Arc<AppState>>) -> Result<String, (StatusCode, String)> {
    let query_str = r#"select account, password from accounts"#;

    let mut rows = sqlx::query(query_str)
        .fetch_all(&app_state.pool)
        .await
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;

    let row = rows.get_mut(0).unwrap();

    let account: String = row.get(0);
    let password: String = row.get(1);

    Ok(format!("Account: {}, Password: {}", account, password))
}
