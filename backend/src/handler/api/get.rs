use axum::http::StatusCode;

pub async fn greeting() -> Result<String, (StatusCode, String)> {
    Ok("Hello world".into())
}
