use axum::{
    Router,
    routing::{get, post},
};
use std::sync::Arc;

use crate::{
    handler::{auth, create_room, join_room, login, logout, rooms, signup},
    router::AppState,
};

pub fn api_router() -> Router<Arc<AppState>> {
    let get_router = Router::new()
        .route("/logout", get(logout))
        .route("/auth", get(auth))
        .route("/create_room", get(create_room))
        .route("/join_room", get(join_room))
        .route("/rooms", get(rooms));

    let post_router = Router::new()
        .route("/signup", post(signup))
        .route("/login", post(login));

    let patch_router = Router::new();

    let delete_router = Router::new();

    let api = Router::new()
        .merge(get_router)
        .merge(post_router)
        .merge(patch_router)
        .merge(delete_router);

    api
}
