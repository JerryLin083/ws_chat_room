use axum_server::tls_rustls::RustlsConfig;
use std::{net::SocketAddr, path::PathBuf, time::Duration};

use crate::{
    db::db_connection, room_manager::RoomManager, router::router, session::SessionManager,
};

pub async fn run() {
    let pool = db_connection().await;
    let session_manager = SessionManager::build(Duration::from_secs(30 * 60));
    let room_manager = RoomManager::build(Duration::from_secs(30 * 60));
    let router = router(pool, session_manager.clone(), room_manager.clone()).await;

    //run session background checker
    let session_manager_for_bg = session_manager.clone();
    session_manager_for_bg.run_checker();

    //tls config
    let config = RustlsConfig::from_pem_file(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("self_signed_cert")
            .join("cert.pem"),
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("self_signed_cert")
            .join("key.pem"),
    )
    .await
    .unwrap();

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000 as u16));

    tracing::info!("Listening on {}...", addr);

    if let Err(err) = axum_server::bind_rustls(addr, config)
        .serve(router.into_make_service())
        .await
    {
        panic!("Server error: {}", err);
    }
}
