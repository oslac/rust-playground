use alu::types::Request;
use alu::types::Response;
use alu::types::Success;

use lambda_runtime::run;
use lambda_runtime::service_fn;
use lambda_runtime::Error;
use lambda_runtime::LambdaEvent;

#[tokio::main]
async fn main() -> Result<(), Error> {
    //                           🤔
    let service = service_fn(|e: LambdaEvent<Request>| handler(e.payload));
    run(service).await?;
    Ok(())
}

async fn handler(req: Request) -> Response {
    log::info!("NEW REQUEST");

    Ok(Success {
        body: format!("Your request body was: {}", req.body),
    })
}
