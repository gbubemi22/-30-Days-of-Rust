# Day 23: Async Rust Fundamentals — solutions

Try the [exercises](../days/23-async-rust-fundamentals/README.md#exercises) first.

## 1. Add another greeting

```rust
tokio::join!(greet("Ada", 20), greet("Linus", 10), greet("Grace", 15));
```

Update the opening line to say three tasks if desired. The order greetings print depends on their delays and scheduling.

## 2. Change a timer

Adjust the delay passed to one call. A shorter delay usually lets that greeting print earlier, though task scheduling can affect exact timing.

## 3. Await a value-producing function

```rust
async fn answer() -> u32 { 42 }

#[tokio::main]
async fn main() {
    let value = answer().await;
    println!("{value}");
}
```

## 4–5. Runtime and awaiting

Tokio's runtime polls futures and provides the timer used in the example. When a future is waiting, `.await` yields control to the executor, which may run another ready task instead of blocking the operating-system thread.
