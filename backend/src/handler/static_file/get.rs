use std::path::PathBuf;

use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
};

pub async fn home() -> Result<impl IntoResponse, (StatusCode, String)> {
    let html_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("static")
        .join("index.html");

    let home_html = tokio::fs::read_to_string(html_path)
        .await
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;

    Ok(Html(home_html))
}
