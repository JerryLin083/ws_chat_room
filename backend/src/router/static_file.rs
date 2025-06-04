use std::sync::Arc;

use axum::{Router, routing::get};
use tower_http::services::ServeDir;

use crate::{handler::home, router::AppState};

pub fn static_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(home))
        .nest_service("/assert", ServeDir::new("static/assert"))
        .fallback(home)
}
