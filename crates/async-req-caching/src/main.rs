use arc::diag;
use std::{error::Error, process::exit};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    color_eyre::install()?;
    diag::setup()?;

    if let Err(e) = arc::serve("0.0.0.0:8080", arc::api()).await {
        eprintln!("ERROR while running the server: {}", e);
        exit(1);
    };

    Ok(())
}
