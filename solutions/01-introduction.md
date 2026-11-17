# Day 1: Introduction to Rust — solutions

Try the [exercises](../days/01-introduction/README.md#exercises) before reading this page. Your exact wording and hobby choice can differ from these examples.

## 1. Print a different message

For example, change the line in `hello_world.rs` to:

```rust
println!("I enjoy drawing.");
```

Compile and run it using the commands from the lesson. The output should be `I enjoy drawing.`

## 2. Print two lines

```rust
fn main() {
    println!("First line");
    println!("Second line");
}
```

Each `println!` ends its output with a newline. `print!` does not, so later output continues on the same line unless a newline is included in the text.

## 3. Explore help

The help text depends on the installed tool version. For example, `rustc --help` lists compiler options, while `cargo --help` lists Cargo commands. Choose any option that interests you; you do not need to memorize the lists.

## 4. Explain the tools

- `rustup` installs and manages Rust toolchains.
- `rustc` compiles Rust source code.
- Cargo builds and runs Rust projects and manages their dependencies.
