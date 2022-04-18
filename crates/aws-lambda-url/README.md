# `aws-lambda-url`

A test with [lambda URLs](https://docs.aws.amazon.com/lambda/latest/dg/lambda-urls.html), where the biggest differences come from the resource configuration in `template.yaml`s compared to regular lambda deployment.

## Instructions

1. `cargo lambda build --release --target=x86_64-unknown-linux-musl`
2. `sam deploy --guided`
3. `sam delete` to clean up the resources.

### Example

```bash
curl -X POST https://{URL-ID}.lambda-url.{REGION}.on.aws -H 'Content-Type: application/json' -d '{"body":"what"}'
```

echoes back the request body:

```bash
{
    "body":"Your request body was: {\"body\":\"what\"}"
}
```

### Disclaimer

*Please note that you may incur AWS charges for deploying the `aws-lambda-url` into your AWS account. To track costs in your AWS account, consider using AWS Cost Explorer and AWS Billing and Cost Management. You can also set up a billing alarm to get notified of unexpected charges.*
