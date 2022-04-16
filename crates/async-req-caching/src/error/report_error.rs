use axum::response::IntoResponse;
use color_eyre::Report;
use reqwest::StatusCode;

/// Wrapper over `eyre` to return it from Axum's handlers.
pub struct ReportError(pub Report);

impl From<Report> for ReportError {
    fn from(e: Report) -> Self {
        Self(e)
    }
}

impl IntoResponse for ReportError {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::INTERNAL_SERVER_ERROR, format!("INTERNAL SERVER ERROR: {:?}", self.0))
            .into_response()
    }
}
