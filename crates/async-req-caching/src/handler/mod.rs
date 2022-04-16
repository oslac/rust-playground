use crate::cache::CachedData;
use crate::data::LatestData;
use axum::{routing::MethodRouter, Extension, Router};
use std::time::Duration;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

pub mod root;

pub fn api() -> Router {
    Router::new()
        .merge(root::handler())
        .layer(TraceLayer::new_for_http())
        .layer(Extension(reqwest::Client::new()))
        .layer(Extension(CachedData::<LatestData>::new(Duration::from_secs(5))))
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()).into_inner())
}

pub fn route(path: &str, method_router: MethodRouter) -> Router {
    Router::new().route(path, method_router)
}
