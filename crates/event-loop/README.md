# `event-loop`

> "*The interface to the system is essentially a pair of channels, and it is a universal interface — everything which is not a batch CLI utility is an event loop*." - [1](https://www.reddit.com/r/rust/comments/uf7yoy/design_patternguidelines_to_architecture_rust_code/i6s4b8x/?context=3)
>
> "*[..] your application has to process messages and make decisions about what it might want to send when something asks it what it wants to send. So you’re no longer pushing messages out of your system. You are just waiting around for something to say, “Hey, what’s your next message?” “Here’s my next message.*” - [2](https://signalsandthreads.com/state-machine-replication-and-why-you-should-care/#002353)

A simple event loop implementation displaying changelogish stream behavior.

## Instructions

1. `RUST_LOG=debug cargo run`
2. In terminal: `--data K` to read from `K`.
3. In terminal: `--data K W` to write `W` to `K`.
