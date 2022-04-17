use async_worker::diag;
use async_worker::http;
use async_worker::on_exit;
use async_worker::routes;
use async_worker::worker;

use axum::Extension;
use std::net::SocketAddr;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = diag::setup()?;
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));

    let (sender, receiver) = mpsc::channel::<String>(10);
    let api = routes::api().layer(Extension(sender));

    // 1. Makes a thread for each task so they're not concurrently in the main
    // thread.
    let server_task = tokio::spawn(http::serve(addr, api));
    let worker_task = tokio::spawn(worker::run(receiver));

    // 2. Drives the tasks to completion concurrently.
    tokio::select! {
        result = server_task => on_exit("server", result),
        result = worker_task => on_exit("worker", result),
    }

    Ok(())
}
