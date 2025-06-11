use axum::{Extension, Router};
use sqlx::{Pool, Postgres};
use std::sync::Arc;
use tokio::sync::mpsc;

mod api;
use api::api_router;
mod static_file;
use static_file::static_router;

use crate::{
    room_manager::{RoomCommand, RoomManager},
    session::SessionManager,
};

pub async fn router(
    pool: Pool<Postgres>,
    session_manager: Arc<SessionManager>,
    room_manager: Arc<RoomManager>,
    db_message_sender: mpsc::Sender<RoomCommand>,
) -> Router {
    let app_state = AppState {
        pool,
        session_manager,
        room_manager,
    };

    let api_router = api_router();
    let static_router = static_router();

    let app = Router::new()
        .merge(static_router)
        .nest("/api", api_router)
        .layer(Extension(db_message_sender))
        .with_state(Arc::new(app_state));

    tracing::info!("Router init...");

    app
}

pub struct AppState {
    pub pool: Pool<Postgres>,
    pub session_manager: Arc<SessionManager>,
    pub room_manager: Arc<RoomManager>,
}
