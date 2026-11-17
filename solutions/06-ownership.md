# Day 6: Ownership — solutions

Try the [exercises](../days/06-ownership/README.md#exercises) first. These are sample solutions; equivalent code is fine.

## 1. Move a `String`

```rust
let first = String::from("move me");
let second = first;
println!("{second}");
```

`second` becomes the owner, so use it after the move.

## 2. Copy an integer

```rust
let first = 12;
let second = first;
println!("{first} and {second}");
```

Both bindings remain usable because `i32` is a `Copy` type.

## 3. Take and return a string

```rust
fn add_period(mut text: String) -> String {
    text.push('.');
    text
}

fn main() {
    let note = String::from("Keep learning");
    let finished_note = add_period(note);
    println!("{finished_note}");
}
```

Ownership moves into the function and then back to the caller as the return value.

## 4. Inspect the move error

After `let second = first`, `second` owns the `String`; `first` is no longer valid. Uncommenting the earlier `println!` attempts to use a moved value, and the compiler reports that ownership was transferred.

## 5. Why the types differ

An `i32` is a small fixed-size value that can be copied directly. A `String` owns a heap allocation; blindly copying its ownership data would let two variables try to manage the same allocation, so Rust moves ownership instead.
