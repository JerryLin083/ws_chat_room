use std::sync::Arc;

use axum::{Router, routing::get};
use tower_http::services::ServeDir;

use crate::{handler::home, router::AppState};

pub fn static_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(home))
        .nest_service("/assets", ServeDir::new("static/assets"))
        .fallback(home)
}
