use crate::types::Chan;

use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Extension;
use axum::Router;
use tracing::instrument;

pub fn api() -> Router {
    Router::new().route("/", get(handler))
}

/// `GET /` for events.
/// Hardcoded for example purposes.
/// A more involved example would send an event to a queue here.
#[instrument(name = "Event Handler triggered", skip(tx))]
async fn handler(Extension(tx): Chan) -> impl IntoResponse {
    match tx.send("HELLO CHANNELS!".to_string()).await {
        Ok(()) => tracing::info!("SEND OK"),
        Err(err) => tracing::error!("SEND ERR: {}", err),
    };

    StatusCode::OK
}
