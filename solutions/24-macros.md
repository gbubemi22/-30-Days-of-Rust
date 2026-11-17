# Day 24: Macros — solutions

Try the [exercises](../days/24-macros/README.md#exercises) first. These are sample implementations.

## 1. Fixed greeting macro

```rust
macro_rules! say_hi {
    () => { println!("Hi!"); };
}

fn main() { say_hi!(); }
```

## 2. Semicolon-separated expressions

```rust
macro_rules! announce_many {
    ($($message:expr);+ $(;)?) => {
        $(println!("Announcement: {}", $message);)+
    };
}

fn main() { announce_many!("One"; "Two"; "Three"); }
```

## 3. Match an enum variant

```rust
enum Status { Ready, Waiting }
let status = Status::Ready;
assert!(matches!(status, Status::Ready));
```

## 4–5. Why and what

Macros can accept or generate syntax that a fixed function signature cannot, such as a variable number of `println!` calls. Declarative macros match token patterns; procedural macros inspect token streams and generate code programmatically.
