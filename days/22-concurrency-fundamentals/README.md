# Day 22: Concurrency Fundamentals

Concurrency means making progress on more than one task during the same period. Threads let parts of a program run independently. Rust's ownership and type systems help prevent data races, and channels or synchronized shared state let threads communicate safely.

## Learning objectives

- start and join a thread;
- move owned data into a thread with `move`;
- send values between threads with a channel;
- share data using `Arc<Mutex<T>>`;
- distinguish a data race from ordinary concurrent execution.

## Prerequisites

Read [Day 21: Serialization and Deserialization](../21-serialization-and-deserialization/README.md), and review ownership and smart pointers.

## Threads and ownership

The standard library's `thread::spawn` starts a new operating-system thread. It returns a join handle; calling `.join()` waits for that thread to finish:

```rust
let handle = std::thread::spawn(|| println!("Hello from a thread"));
handle.join().expect("thread panicked");
```

A spawned thread may run after the function that created it continues. Use the `move` keyword when the closure should take ownership of captured values, so they remain valid for the thread.

## Message passing with channels

Channels send values from one thread to another. `std::sync::mpsc::channel` creates a sender and receiver. The sender can move to a worker thread; the receiver reads messages:

```rust
use std::sync::mpsc;

let (sender, receiver) = mpsc::channel();
std::thread::spawn(move || sender.send(String::from("done")).unwrap());
let message = receiver.recv().unwrap();
```

Message passing is often a clear way to transfer ownership of data between tasks.

## Shared state with `Arc` and `Mutex`

When threads need shared mutable state, `Arc<T>` provides thread-safe shared ownership, and `Mutex<T>` allows one thread at a time to access the protected value. Locking returns a guard; when the guard goes out of scope, the lock is released:

```rust
use std::sync::{Arc, Mutex};

let total = Arc::new(Mutex::new(0));
let shared = Arc::clone(&total);
*shared.lock().expect("lock poisoned") += 1;
```

The mutex can be poisoned if a thread panics while holding its lock. The example uses `expect` to keep the focus on sharing; a full program should decide how to respond to lock poisoning.

## Practical example

Open [`examples/workers.rs`](examples/workers.rs):

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let completed = Arc::new(Mutex::new(Vec::new()));
    let mut handles = Vec::new();

    for worker in vec!["Ada", "Linus"] {
        let completed = Arc::clone(&completed);
        handles.push(thread::spawn(move || {
            let mut names = completed.lock().expect("lock poisoned");
            names.push(worker);
        }));
    }

    for handle in handles {
        handle.join().expect("worker panicked");
    }

    let mut names = completed.lock().expect("lock poisoned");
    names.sort();
    println!("Completed workers: {names:?}");
}
```

Compile and run from the repository root:

```sh
rustc days/22-concurrency-fundamentals/examples/workers.rs -o workers
./workers
```

Expected output:

```text
Completed workers: ["Ada", "Linus"]
```

Each worker owns its name and an `Arc` clone. The mutex protects the shared vector while a name is added. Threads may finish in either order, so the main thread sorts the names before printing.

## Common mistakes

- **Borrowing a local value into a thread that may outlive it:** Use `move` to transfer owned data or share it through an `Arc`.
- **Using `Rc` across threads:** `Rc` is not thread-safe. Use `Arc` for shared ownership across threads.
- **Accessing shared mutable data without synchronization:** Use a mutex or an appropriate atomic type to coordinate access.
- **Holding a lock longer than necessary:** Keep the lock guard's scope short so other threads can make progress.
- **Assuming thread completion order:** Scheduling is nondeterministic. Do not rely on a particular order unless you coordinate it.
- **Ignoring a failed `join`:** A worker may panic. Decide how the main thread should respond.

## Exercises

Try these before reading the [Day 22 solutions](../../solutions/22-concurrency-fundamentals.md).

1. Spawn a thread that prints a message, then join it.
2. Use a channel to send a number from a worker to the main thread.
3. Modify the shared vector example to add a third worker.
4. Use `Arc<Mutex<i32>>` to let two workers each increment a shared counter five times.
5. Explain the difference between transferring a message and sharing mutable state.

## Further reading

- [The Rust Programming Language: Fearless Concurrency](https://doc.rust-lang.org/book/ch16-00-concurrency.html)
- [Rust By Example: Threads](https://doc.rust-lang.org/rust-by-example/std_misc/threads.html)
- [Standard library: `std::sync`](https://doc.rust-lang.org/std/sync/index.html)

## Course navigation

- [Course index](../../README.md)
- Previous: [Day 21: Serialization and Deserialization](../21-serialization-and-deserialization/README.md)
- Next: [Day 23: Async Rust Fundamentals](../23-async-rust-fundamentals/README.md)
