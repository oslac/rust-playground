# `async-req-caching`

This is a working example of the [request coalescing pattern](https://fasterthanli.me/articles/request-coalescing-in-async-rust) in async Rust.

## Instructions

1. `RUST_LOG=debug,tower_http=debug cargo run --release`
2. `oha -n 10 -c 5 http://localhost:8080` runs 10 requests with 5 clients, and it should return with a status code distribution split equally between `200` and `500`.
3. Running the same command a second time should return only `200 OK`s.

Using other HTTP load generator tools, like [rakyll/hey](https://github.com/rakyll/hey), should also return the same results.

Using `curl` at the cold start should also cause the [receiver](https://github.com/oslac/rs-scribbles/blob/9b12556df887a5010af7161575b023791a1e8d16/crates/async-req-caching/src/cache.rs#L118) to return an error (and if you keep calling it, the actual response fetched from `jsonplaceholder.com`)

```bash
$ curl http://localhost:8080

INTERNAL SERVER ERROR:
   0: ERROR: IN-FLIGHT-REQUEST DIED
```
