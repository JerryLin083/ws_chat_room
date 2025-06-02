use axum_server::tls_rustls::RustlsConfig;
use std::{net::SocketAddr, path::PathBuf};

use crate::{db::db_connection, router::router};

pub async fn run() {
    let pool = db_connection().await;
    let router = router(pool).await;

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
