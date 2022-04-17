use crate::types::AppError;

use tokio::sync::mpsc::Receiver;
use tracing::instrument;

enum TaskResult {
    Completed,
    Empty,
}

/// Starts the asyncronous workflow.
///
/// The argument could be pool of database connections in a real
/// implementation, or whatever the unit of work is that needs to be executed
/// asynchronously & in parallel with server.
pub async fn run<T>(rx: Receiver<T>) -> AppError<()> {
    worker_loop(rx).await
}

async fn worker_loop<T>(mut rx: Receiver<T>) -> AppError<()> {
    loop {
        match try_task(&mut rx).await {
            TaskResult::Completed => {
                // Logic goes here
                tracing::info!("Task: completed")
            }
            TaskResult::Empty => {
                // And here
                tracing::info!("Task: empty")
            }
        }
    }
}

#[instrument(name = "Waiting for Task", skip_all)]
async fn try_task<T>(rx: &mut Receiver<T>) -> TaskResult {
    // This blocks until Some(event)
    match rx.recv().await {
        Some(_) => {
            // Also stuff logic here
            tracing::debug!("New task received");
            TaskResult::Completed
        }
        None => {
            // And here
            tracing::debug!("Channel: closed");
            TaskResult::Empty
        }
    }
}
