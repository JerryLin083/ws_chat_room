use axum::{Router, routing::get};
use std::sync::Arc;

use crate::{handler::greeting, router::AppState};

pub fn api_router() -> Router<Arc<AppState>> {
    let get_router = Router::new().route("/", get(greeting));

    let post_router = Router::new();

    let patch_router = Router::new();

    let delete_router = Router::new();

    let api = Router::new()
        .merge(get_router)
        .merge(post_router)
        .merge(patch_router)
        .merge(delete_router);

    api
}
