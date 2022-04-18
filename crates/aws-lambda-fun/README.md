# `aws-lambda-fun`

This example is meant to instruct what are the barebones project & configuration requirements for deploying a hello-worldish Rust application to AWS Lambda; there are not many requirements.

- `Cargo.toml` contains the minimal required libs. for starting lambda development.
- `template.yaml` contains the minimal SAM template to deploy the application.
- [lambda-http/examples](https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/lambda-http/examples)

## Instructions

1. `cargo lambda build --release --target=x86_64-unknown-linux-musl`
2. `sam deploy`
3. `curl <YOUR_LAMBDA_ADDRESS>` should return `Hello AWS Lambda HTTP request`.
4. `sam delete` to clean up the resources.

## Disclaimer

*Please note that you may incur AWS charges for deploying the `aws-lambda-fun` into your AWS account. To track costs in your AWS account, consider using AWS Cost Explorer and AWS Billing and Cost Management. You can also set up a billing alarm to get notified of unexpected charges.*
