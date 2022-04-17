# `async-worker`

Example of how to **structure** async Rust application with a set of *asynchronous tasks* that execute *in parallel*.

Combined with channels, this can be used avoid lock contention scenarios, or to recover the property of transactionality when working with events.
