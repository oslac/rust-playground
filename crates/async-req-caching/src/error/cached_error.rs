use super::report_error::ReportError;
use color_eyre::Report;
use tokio::sync::broadcast;

/// A string representation of caching error in order to guarantee that all
/// "errors" we might ever receive are `Send + Sync + Clone` & implement
/// `std::error::Error` (bounding this otherwise would be annoying / hard).
#[derive(Debug, Clone, thiserror::Error)]
#[error("ERROR: {inner}")]
pub struct CachedError {
    inner: String,
}

impl CachedError {
    pub fn new<E: std::fmt::Display>(e: E) -> Self {
        Self { inner: e.to_string() }
    }
}

impl From<broadcast::error::RecvError> for CachedError {
    fn from(e: broadcast::error::RecvError) -> Self {
        CachedError::new(e)
    }
}

impl From<Report> for CachedError {
    fn from(e: Report) -> Self {
        Self::new(e)
    }
}

impl From<CachedError> for ReportError {
    fn from(e: CachedError) -> Self {
        ReportError(e.into())
    }
}
