# Day 28: Code Quality and Documentation

Readable code is easier to change and review. Rust's toolchain includes `rustfmt` for formatting, Clippy for lints, and Rustdoc for API documentation. Today you will use these tools and write documentation that explains how a function should be used.

## Learning objectives

- format Rust code with rustfmt;
- run Clippy and understand that lints are suggestions to review;
- write documentation comments and examples;
- generate API documentation with Rustdoc;
- recognize a few common idiomatic Rust choices.

## Prerequisites

Read [Day 27: Command-Line Applications](../27-command-line-applications/README.md). This lesson uses a small Cargo library project.

## Formatting with rustfmt

`rustfmt` formats Rust source files. In a Cargo project, use:

```sh
cargo fmt --all
```

To check formatting without changing files:

```sh
cargo fmt --all -- --check
```

Formatting makes layout consistent; it does not prove that code is correct.

## Clippy

Clippy checks for common mistakes and opportunities to make code clearer or more idiomatic:

```sh
cargo clippy --all-targets -- -D warnings
```

Read each lint and decide whether it applies. A lint is a prompt to review code, not a reason to silence a warning without understanding it.

## Documentation comments and Rustdoc

Use `///` comments for public items. They support Markdown and appear in generated API documentation. A short example is especially useful:

```rust
/// Returns the square of `number`.
///
/// # Examples
///
/// ```
/// assert_eq!(quality_kit::square(4), 16);
/// ```
pub fn square(number: i32) -> i32 { number * number }
```

Run `cargo doc --open` to build and view the documentation. Code blocks marked as Rust examples can be compiled as documentation tests when you run `cargo test`.

## Practical example

The example library is in [`examples/quality-kit/`](examples/quality-kit/). From the repository root, run:

```sh
cargo fmt --manifest-path days/28-code-quality-and-documentation/examples/quality-kit/Cargo.toml -- --check
cargo clippy --manifest-path days/28-code-quality-and-documentation/examples/quality-kit/Cargo.toml --all-targets -- -D warnings
cargo test --manifest-path days/28-code-quality-and-documentation/examples/quality-kit/Cargo.toml
cargo doc --manifest-path days/28-code-quality-and-documentation/examples/quality-kit/Cargo.toml --no-deps
```

The library documents `square` and includes a unit test plus a documentation example. A successful run ends with the test passing and Rustdoc generating the crate documentation.

## Common mistakes

- **Treating formatting as a correctness check:** rustfmt only standardizes layout.
- **Applying a Clippy suggestion without understanding it:** Check what the lint means and whether the suggested change fits your code.
- **Writing docs that repeat the function name:** Explain purpose, inputs, edge cases, and examples instead.
- **Leaving examples out of sync:** Documentation examples should compile and demonstrate current behavior.
- **Silencing warnings broadly:** Fix the cause where practical; narrow any allow attribute and explain why it is needed.

## Exercises

Try these before reading the [Day 28 solutions](../../solutions/28-code-quality-and-documentation.md).

1. Add documentation for a function you wrote earlier, including an example.
2. Run rustfmt on a deliberately misformatted copy of the example.
3. Run Clippy and explain one lint it reports or confirm that it reports none.
4. Add a documentation example that checks a boundary case.
5. Explain what each of rustfmt, Clippy, and Rustdoc does.

## Further reading

- [rustfmt](https://github.com/rust-lang/rustfmt)
- [Clippy lint list](https://doc.rust-lang.org/clippy/lints.html)
- [The Rustdoc Book](https://doc.rust-lang.org/rustdoc/)

## Course navigation

- [Course index](../../README.md)
- Previous: [Day 27: Command-Line Applications](../27-command-line-applications/README.md)
- Next: [Day 29: Build a CLI Task Manager](../29-cli-task-manager/README.md)
