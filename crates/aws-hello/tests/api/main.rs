use aws_hello::hello;
use lambda_http::{Body, IntoResponse, Request};

#[tokio::test]
async fn test_hello() {
    // GIVEN
    let request = Request::default();
    let expected = Body::Text("Hello AWS Lambda HTTP request".to_string());
    // WHEN
    let response = hello::handler(request).await.unwrap().into_response();
    // THEN
    let actual = response.body();
    assert_eq!(expected, *actual)
}
