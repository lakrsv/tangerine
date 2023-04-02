//! A basic HTTP server, to test overriding a container's ENTRYPOINT.
use std::{env, net::SocketAddr, path::PathBuf};

use aes_gcm::{aead::Aead, Nonce, Aes256Gcm, aes::Aes256, KeyInit};
use axum::{Router, routing::{get, delete}, extract::Path, body::Bytes};

use tokio::signal;

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new().route("/client/:id", get(get_client_tangerine));

    // run it
    let addr = SocketAddr::from(([0, 0, 0, 0], 80));
    println!("server is ready");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

async fn get_client_tangerine(Path(id): Path<String>) -> Bytes {
    let crypto = Aes256Gcm::new_from_slice(test_images::constants::ENCRYPTION_KEY.as_bytes()).unwrap();
    let nonce = Nonce::from_slice(test_images::constants::NONCE.as_bytes());
    let plain_command = "!TANGERINE
    echo \"Hello World\" >> hello_world.txt
    !TANGERINE
    ";
    let encrypted_command = crypto.encrypt(nonce, plain_command.as_bytes()).unwrap();
    Bytes::from(encrypted_command)
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    println!("signal received, starting graceful shutdown");
}