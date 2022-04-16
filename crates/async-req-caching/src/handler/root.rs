use super::route;
use crate::{cache::CachedData, data::LatestData, error::report_error::ReportError};
use axum::{response::IntoResponse, routing::get, Extension, Json, Router};
use color_eyre::Report;
use serde::Serialize;
use tracing::instrument;

pub fn handler() -> Router {
    route("/", get(root))
}

#[derive(Serialize)]
struct Response {
    data: String,
}

/// Handler for `GET /`
#[instrument(skip(client, cached))]
async fn root(
    client: Extension<reqwest::Client>,
    cached: Extension<CachedData<LatestData>>,
) -> Result<impl IntoResponse, ReportError> {
    let LatestData(data) = cached
        .get_cached(|| {
            Box::pin(async move {
                // Fetch some new data here:
                let data = fetch(&client).await?;
                Ok::<_, Report>(LatestData(data))
            })
        })
        .await?;
    Ok(Json(Response { data }))
}

async fn fetch(client: &reqwest::Client) -> Result<String, Report> {
    // Strictly containing only the fetching logic
    let url = "https://jsonplaceholder.typicode.com/posts/1";
    let body = client.get(url).send().await?.text().await?;
    tracing::debug!("BODY {:?}", body);
    Ok(body)
}
