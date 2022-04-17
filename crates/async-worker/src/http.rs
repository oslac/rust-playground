use crate::types::AppError;

use axum::Router;
use axum::Server;
use std::net::SocketAddr;
use tokio::signal;

pub async fn serve(addr: SocketAddr, api: Router) -> AppError<()> {
    tracing::debug!("LISTENING ON: {}", addr);
    Server::bind(&addr).serve(api.into_make_service()).with_graceful_shutdown(signal()).await?;
    Ok(())
}

async fn signal() {
    let ctrl_c = async {
        signal::ctrl_c().await.expect("Ctrl+C handler installed");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("Installing signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    println!("Signal received: starting shutdown");
}
