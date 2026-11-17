# Day 28: Code Quality and Documentation — solutions

Try the [exercises](../days/28-code-quality-and-documentation/README.md#exercises) first.

## 1. Document a function

```rust
/// Converts minutes to whole hours, discarding any remaining minutes.
///
/// # Examples
///
/// ```
/// assert_eq!(whole_hours(125), 2);
/// ```
pub fn whole_hours(minutes: u32) -> u32 { minutes / 60 }
```

## 2–3. Formatting and Clippy

Run the documented Cargo commands from the example project. rustfmt changes layout; Clippy reports lints for you to review. The exact lint output can depend on the installed toolchain.

## 4. Document a boundary

Add examples for boundary values such as zero and the largest supported input. Documentation examples can be compiled during `cargo test`.

## 5. Tool roles

rustfmt formats source, Clippy identifies common lint patterns, and Rustdoc builds documentation from source comments and Markdown.
