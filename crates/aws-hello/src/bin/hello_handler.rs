use aws_hello::hello;
use lambda_http::{run, service_fn, Error};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let service = service_fn(hello::handler);
    run(service).await
}
