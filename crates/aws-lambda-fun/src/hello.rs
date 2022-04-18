use lambda_http::{Error, IntoResponse, Request};

/// This is the event handler.
pub async fn handler(_event: Request) -> Result<impl IntoResponse, Error> {
    // Extract some useful information from the request
    // Return something that implements IntoResponse.
    // It will be serialized to the right response event automatically by the
    // runtime
    Ok("Hello AWS Lambda HTTP request".into_response())
}
