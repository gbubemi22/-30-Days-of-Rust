# Day 23: Async Rust Fundamentals

Asynchronous programming lets a task pause while it waits for work, allowing an executor to run other ready tasks. Rust's `async` and `.await` syntax describe this behavior, while a runtime such as Tokio drives futures and provides timers and asynchronous I/O.

## Learning objectives

- explain what a future represents;
- define and call an `async fn`;
- use `.await` to wait for an async operation;
- explain why async code needs an executor/runtime;
- run a small Tokio example with Cargo.

## Prerequisites

Read [Day 22: Concurrency Fundamentals](../22-concurrency-fundamentals/README.md). You should understand functions and basic concurrency. This lesson uses the Tokio crate.

## Futures and async functions

An async function does not immediately perform all its work when called. It returns a **future**, a value representing work that may complete later. An executor polls futures and runs them when they can make progress.

Use `async fn` to define asynchronous work and `.await` to pause that async task until another future is ready:

```rust
async fn greeting() -> String {
    String::from("Hello from async Rust")
}
```

Calling `greeting()` creates a future. It must be awaited inside an async context and driven by an executor. Rust's standard library defines the `Future` trait but does not include a general-purpose runtime with timers and I/O; applications commonly choose a runtime crate for those services.

## What an executor does

An executor schedules futures, checks whether they can make progress, and resumes them when they are ready. `.await` does not create a new operating-system thread by itself. Async programming is especially useful for I/O-bound work that spends time waiting, such as many network connections. It is not automatically a faster way to perform CPU-heavy calculations.

## Tokio example

The example project is in [`examples/tokio-greeting/`](examples/tokio-greeting/). Tokio's `#[tokio::main]` macro sets up a runtime for the async `main` function. Run it from the repository root:

```sh
cargo run --manifest-path days/23-async-rust-fundamentals/examples/tokio-greeting/Cargo.toml
```

The first build downloads the declared crate if needed. Expected output:

```text
Starting two async tasks...
Hello, Ada!
Hello, Linus!
Both tasks finished.
```

The greeting functions wait on Tokio timers. `tokio::join!` waits for both futures in the same task; it does not mean they each use a separate operating-system thread. The greeting results are printed afterward in the order of the `join!` arguments, so the output remains predictable even though both waits overlap.

## Common mistakes

- **Calling an async function and expecting it to run:** Calling it creates a future. Await it in an async context so the executor can drive it.
- **Using `.await` in an ordinary function:** Await is only valid in async contexts.
- **Assuming async creates a thread:** Async tasks are scheduled by a runtime and can share a thread.
- **Blocking an async executor thread:** Long blocking work can prevent other tasks from progressing. Use an appropriate blocking strategy for CPU-heavy or blocking operations.
- **Using a runtime for every program:** Small synchronous programs may be simpler with ordinary functions and threads.

## Exercises

Try these before reading the [Day 23 solutions](../../solutions/23-async-rust-fundamentals.md).

1. Add a third async greeting and await it with the other tasks.
2. Change one timer duration and observe which greeting completes first.
3. Write an async function that returns a number, then await it from `main`.
4. Explain what the Tokio runtime does for this program.
5. Explain why `.await` does not necessarily block an operating-system thread until completion.

## Further reading

- [The Rust Async Book](https://rust-lang.github.io/async-book/)
- [Tokio tutorial: Setup](https://tokio.rs/tokio/tutorial/setup)
- [Tokio tutorial: Hello Tokio](https://tokio.rs/tokio/tutorial/hello-tokio)

## Course navigation

- [Course index](../../README.md)
- Previous: [Day 22: Concurrency Fundamentals](../22-concurrency-fundamentals/README.md)
- Next: [Day 24: Macros](../24-macros/README.md)
