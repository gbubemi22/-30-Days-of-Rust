# Day 30: Final Project and Next Steps

You have reached the final day. The CLI task manager brings together ideas from across the course: types, ownership, borrowing, enums and errors, collections, traits, modules, external crates, files, tests, and command-line input. Today you will review the project, practice explaining its design, and choose a path for continuing your Rust learning.

## Learning objectives

- explain how the task manager is organized;
- run its checks and describe what each one verifies;
- review core Rust concepts used in the project;
- document a project and identify useful extensions;
- choose a next step based on your interests.

## Final project

The completed project is [`projects/cli-task-manager/`](../../projects/cli-task-manager/). Follow its README to add, list, and complete tasks. Its library contains the data model, JSON persistence, command handling, and tests; `main` handles process arguments, configuration, and exit codes.

Run the project's quality checks from its directory:

```sh
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
cargo doc --no-deps
```

Formatting checks layout, Clippy checks for lints, tests check behavior, and Rustdoc builds API documentation. These checks complement one another; no single command proves a program has no bugs.

## Review the concepts

- **Ownership and borrowing:** Which function owns a task list? Which functions borrow tasks?
- **Types and enums:** How does `Result` communicate an operation that may fail?
- **Collections:** Why is a `Vec<Task>` a good fit for an ordered task list?
- **Traits:** Which Serde traits are derived for `Task`?
- **Modules and packages:** Why is command logic in a library instead of all in `main`?
- **Errors and tests:** What happens if the JSON is malformed or task number zero is given?
- **Crates:** Which dependency provides JSON serialization, and how can you inspect its dependency tree?

Try to answer each question in your own words, then point to the relevant code and lesson.

## Possible extensions

- remove or reopen tasks;
- add due dates, priorities, or tags;
- support filtering and sorting;
- add configuration for the data path;
- improve help text and argument parsing;
- add integration tests and test file-system edge cases;
- export tasks to CSV or another format.

Add one feature at a time. Update tests and documentation when behavior changes.

## Continue learning

- Revisit [The Rust Programming Language](https://doc.rust-lang.org/book/) and complete its projects.
- Use [Rust By Example](https://doc.rust-lang.org/rust-by-example/) to explore syntax with small runnable examples.
- Read the [standard library documentation](https://doc.rust-lang.org/std/) when you need a type or method.
- Pick a small project that interests you, and read crate documentation before adding dependencies.
- Explore testing, unsafe Rust, embedded development, systems programming, or async networking as your goals require.

The best next topic depends on what you want to build. Keep projects small enough to finish, and use compiler messages as feedback while you learn.

## Course navigation

- [Course index](../../README.md)
- Previous: [Day 29: Build a CLI Task Manager](../29-cli-task-manager/README.md)
- Next: none — this is Day 30.
