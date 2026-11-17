# Day 16: Modules and Packages — solutions

Try the [exercises](../days/16-modules-and-packages/README.md#exercises) first. These are example layouts and explanations.

## 1. A public greeting module

```rust
mod greetings {
    pub fn hello() { println!("Hello!"); }
}
fn main() { greetings::hello(); }
```

## 2. A private field

```rust
mod profile {
    pub struct Person { name: String }
}
```

Outside `profile`, callers can name the public type but cannot read or initialize its private field. The module can provide public methods or a constructor to control access.

## 3. Import a standard type

```rust
use std::collections::HashMap;
let mut counts = HashMap::new();
counts.insert("apples", 3);
```

## 4. Two-file module layout

`main.rs` declares `mod greetings;` and calls `greetings::hello()`. The sibling file `greetings.rs` defines `pub fn hello()`. Compile the main file; Rust loads the module file from the same directory.

## 5. Terms

A package is a Cargo project described by a manifest. A crate is one compilation unit within it. A module organizes code and visibility inside a crate.
