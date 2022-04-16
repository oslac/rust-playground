use axum::{Router, Server};
use std::{error::Error, net::SocketAddr};

/// Generic caching implementation for some data.
pub mod cache;
/// The some data referenced above.
pub mod data;
/// Error types
pub mod error;
/// Route handler
pub mod handler;
/// Tracing for debugging
pub mod diag;
/// Counter
pub mod doom;

pub use handler::api;

/// Given a socket address and an api, serves it.
pub async fn serve(addr: &str, api: Router) -> Result<(), Box<dyn Error>> {
    let addr: SocketAddr = addr.parse()?;
    tracing::info!("LISTENING ON: http://{}", addr);
    Server::bind(&addr).serve(api.into_make_service()).await?;
    Ok(())
}
