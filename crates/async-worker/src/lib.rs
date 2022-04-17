/// The background worker.
pub mod worker;
pub mod types;
pub mod routes;
pub mod http;
pub mod diag;

pub fn on_exit(
    task_name: &str,
    result: Result<Result<(), impl std::fmt::Debug + std::fmt::Display>, tokio::task::JoinError>,
) {
    match result {
        Ok(Ok(_)) => {
            tracing::info!("{} EXIT (SUCCESS)", task_name)
        }
        Ok(Err(err)) => {
            tracing::error!("{} EXIT (FAILED)", err)
        }
        Err(err) => {
            tracing::error!("{} FAILED TO COMPLETE", err)
        }
    }
}
