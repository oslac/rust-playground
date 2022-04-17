use axum::Extension;
use tokio::sync::mpsc::Sender;

pub type Chan = Extension<Sender<String>>;
pub type AppError<T> = Result<T, Box<dyn std::error::Error + Send + Sync + 'static>>;
