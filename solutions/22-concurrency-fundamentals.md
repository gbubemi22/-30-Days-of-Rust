# Day 22: Concurrency Fundamentals — solutions

Try the [exercises](../days/22-concurrency-fundamentals/README.md#exercises) first. These are sample approaches.

## 1. Spawn and join

```rust
let handle = std::thread::spawn(|| println!("Worker started"));
handle.join().expect("worker panicked");
```

## 2. Send a number with a channel

```rust
use std::sync::mpsc;
let (sender, receiver) = mpsc::channel();
std::thread::spawn(move || sender.send(42).unwrap());
let number = receiver.recv().unwrap();
println!("Received {number}");
```

## 3. Add another worker

Add another name to the vector used by the `for` loop. Each iteration clones the `Arc`, and the final sorted list includes the new name.

## 4. Shared counter

```rust
use std::sync::{Arc, Mutex};
use std::thread;

let count = Arc::new(Mutex::new(0));
let mut handles = Vec::new();
for _ in 0..2 {
    let count = Arc::clone(&count);
    handles.push(thread::spawn(move || {
        for _ in 0..5 {
            *count.lock().unwrap() += 1;
        }
    }));
}
for handle in handles { handle.join().unwrap(); }
assert_eq!(*count.lock().unwrap(), 10);
```

## 5. Message passing and shared state

Message passing transfers values between tasks through a channel. Shared state lets tasks access one common value, so mutation must be coordinated with a mutex or another synchronization primitive.
