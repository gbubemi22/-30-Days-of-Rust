# CLI Task Manager

A small Rust command-line task manager with JSON persistence. It supports `add`, `list`, and `done` commands.

## Usage

```sh
cargo run -- add "Read about ownership"
cargo run -- list
cargo run -- done 1
```

By default, tasks are stored in `tasks.json` in the current directory. Set `TASKS_FILE` to use another path. The project does not include private configuration or real personal data.

## Check the project

```sh
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
```
