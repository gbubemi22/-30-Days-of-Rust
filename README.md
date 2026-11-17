# 30 Days of Rust

A beginner-friendly course for learning Rust through short explanations, runnable examples, exercises, and a CLI project. This is community learning material and is not an official Rust project.

## Who this course is for

This course is for people who are new to Rust. No prior systems-programming experience is assumed. The lessons move from basic syntax through ownership, collections, concurrency, tooling, and a practical project.

## Before you begin

Install the stable Rust toolchain with [rustup](https://www.rust-lang.org/tools/install). You will use `rustc` for small standalone files and Cargo for package examples, dependencies, and the final project. Lessons are Markdown and can be read directly on GitHub.

## Course outline and progress

All 30 lessons, examples, exercises, and separate solutions are complete. Local Markdown links, standalone examples, Cargo examples, formatting, Clippy, documentation generation, and workspace tests have been checked.

- [x] All 30 lesson pages and navigation links are present.
- [x] Every day has a runnable example or capstone project instructions.
- [x] Exercises and separate worked solutions are present for all 30 days.
- [x] Workspace formatting, Clippy, tests, and Rustdoc checks pass.
- [x] Internal Markdown links resolve.

### Stage 1: Rust foundations

1. [Introduction to Rust](days/01-introduction/README.md)
2. [Variables and Mutability](days/02-variables-and-mutability/README.md)
3. [Data Types](days/03-data-types/README.md)
4. [Operators and Control Flow](days/04-operators-and-control-flow/README.md)
5. [Functions](days/05-functions/README.md)
6. [Ownership](days/06-ownership/README.md)

### Stage 2: Core Rust

7. [References and Borrowing](days/07-references-and-borrowing/README.md)
8. [Strings and Slices](days/08-strings-and-slices/README.md)
9. [Structs and Methods](days/09-structs-and-methods/README.md)
10. [Enums and Pattern Matching](days/10-enums-and-pattern-matching/README.md)
11. [Collections](days/11-collections/README.md)
12. [Error Handling](days/12-error-handling/README.md)

### Stage 3: Intermediate Rust

13. [Generics](days/13-generics/README.md)
14. [Traits](days/14-traits/README.md)
15. [Lifetimes](days/15-lifetimes/README.md)
16. [Modules and Packages](days/16-modules-and-packages/README.md)
17. [Iterators and Closures](days/17-iterators-and-closures/README.md)
18. [Smart Pointers](days/18-smart-pointers/README.md)

### Stage 4: Broader Rust programming

19. [Testing and Debugging](days/19-testing-and-debugging/README.md)
20. [File Handling](days/20-file-handling/README.md)
21. [Serialization and Deserialization](days/21-serialization-and-deserialization/README.md)
22. [Concurrency Fundamentals](days/22-concurrency-fundamentals/README.md)
23. [Async Rust Fundamentals](days/23-async-rust-fundamentals/README.md)
24. [Macros](days/24-macros/README.md)

### Stage 5: Practical Rust

25. [Working with External Crates](days/25-working-with-external-crates/README.md)
26. [Environment and Configuration](days/26-environment-and-configuration/README.md)
27. [Command-Line Applications](days/27-command-line-applications/README.md)
28. [Code Quality and Documentation](days/28-code-quality-and-documentation/README.md)
29. [Build a CLI Task Manager](days/29-cli-task-manager/README.md)
30. [Final Project and Next Steps](days/30-final-project-and-next-steps/README.md)

## Run examples

Standalone examples can be compiled from the repository root with `rustc`, as in Day 1:

```sh
rustc days/01-introduction/examples/hello_world.rs -o hello_world
./hello_world
```

Examples that use external crates or Cargo features include their own `Cargo.toml`. Their lesson pages give the matching `cargo run` or `cargo test` command. Cargo projects are members of the root workspace. See [`examples/README.md`](examples/README.md) for the example layout.

## Exercises and solutions

Attempt the exercises in each lesson before opening the corresponding file in [`solutions/`](solutions/README.md). The capstone task manager is in [`projects/cli-task-manager/`](projects/cli-task-manager/README.md).

## Checks

From the repository root, run the workspace checks:

```sh
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

These checks cover Cargo workspace packages. Standalone examples are documented individually and can be compiled with the commands in their lessons.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for lesson format and contribution guidance. Contributions should use original explanations and examples, keep lesson navigation accurate, and clearly mark incomplete content.

## License

The material in this repository is licensed under the [MIT License](LICENSE), unless a file or third-party dependency states otherwise. Rust, Cargo, and the Rust project are trademarks and projects of their respective maintainers; this course is community material.
