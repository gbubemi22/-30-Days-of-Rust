# Day 29: Build a CLI Task Manager

In this project day, you will put together command-line input, structs, collections, JSON, file handling, error handling, and tests. The finished task manager stores tasks in a JSON file and supports adding, listing, and completing tasks.

## Learning objectives

- model a task with a struct;
- load and save task data as JSON;
- parse simple CLI subcommands and arguments;
- report invalid input and file errors without panicking;
- test core behavior separately from the terminal interface.

## Prerequisites

Review [Day 12: Error Handling](../12-error-handling/README.md), [Day 19: Testing and Debugging](../19-testing-and-debugging/README.md), [Day 21: Serialization and Deserialization](../21-serialization-and-deserialization/README.md), and [Day 27: Command-Line Applications](../27-command-line-applications/README.md).

## Plan the program

The implementation lives in [`projects/cli-task-manager/`](../../projects/cli-task-manager/). Its `Task` struct stores a description and completion flag. A library module handles persistence and commands; `main` chooses the data file, passes in arguments, and reports errors.

The task file defaults to `tasks.json` in the current directory. Set `TASKS_FILE` to choose another path. The program uses JSON so task descriptions can safely contain punctuation without inventing a custom delimiter format.

## Run the task manager

From the repository root, add two tasks:

```sh
cargo run --manifest-path projects/cli-task-manager/Cargo.toml -- add "Review ownership"
cargo run --manifest-path projects/cli-task-manager/Cargo.toml -- add "Practice borrowing"
```

List them:

```sh
cargo run --manifest-path projects/cli-task-manager/Cargo.toml -- list
```

Expected output:

```text
1. [ ] Review ownership
2. [ ] Practice borrowing
```

Mark the first task complete and list again:

```sh
cargo run --manifest-path projects/cli-task-manager/Cargo.toml -- done 1
cargo run --manifest-path projects/cli-task-manager/Cargo.toml -- list
```

```text
1. [x] Review ownership
2. [ ] Practice borrowing
```

The first build downloads the Serde dependencies if they are not already cached. Use `TASKS_FILE` to keep experiments separate or to choose a temporary file.

## How the pieces fit

- The `Task` struct models each item, and Serde derives convert tasks to and from JSON.
- `load_tasks` treats a missing data file as an empty task list and reports malformed JSON as invalid data.
- `save_tasks` writes the updated collection.
- `run` validates commands and task numbers, then writes command output to a writer so it can be tested without launching a process.
- Unit tests cover adding, listing, completing, missing files, and malformed files.

## Common mistakes

- **Using task number zero:** Task numbers displayed to the user start at one; the program validates this before converting to a vector index.
- **Assuming the data file exists:** The first `add` starts with an empty list when the file is missing.
- **Overwriting data with malformed JSON:** Load errors are returned instead of silently discarding the file.
- **Hard-coding the data path:** `TASKS_FILE` makes it possible to choose a separate file for testing or another run location.
- **Putting all logic in `main`:** Keeping behavior in a library function makes it easier to test.

## Exercises

Try these before reading the [Day 29 solutions](../../solutions/29-cli-task-manager.md).

1. Add a `remove N` command that deletes the numbered task.
2. Add a command that clears the completed flag for a task.
3. Add a due-date field and update serialization tests.
4. Add a test for an unknown command and one for task number zero.
5. Explain why the command handler accepts a writer rather than printing directly.

## Further reading

- [The Rust Programming Language: I/O Project](https://doc.rust-lang.org/book/ch12-00-an-io-project.html)
- [Serde derive guide](https://serde.rs/derive.html)
- [Cargo Book: Testing](https://doc.rust-lang.org/cargo/commands/cargo-test.html)

## Course navigation

- [Course index](../../README.md)
- Previous: [Day 28: Code Quality and Documentation](../28-code-quality-and-documentation/README.md)
- Next: [Day 30: Final Project and Next Steps](../30-final-project-and-next-steps/README.md)
