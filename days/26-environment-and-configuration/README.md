# Day 26: Environment and Configuration

Programs often need settings that differ between a developer's computer and another environment. Environment variables and configuration files provide those settings without hard-coding them into source. Secrets such as tokens need extra care: keep them out of source control and avoid printing them.

## Learning objectives

- read environment variables with `std::env`;
- load simple configuration values from a file;
- combine file defaults with environment overrides;
- handle missing or malformed settings;
- keep secrets out of source code, logs, and version control.

## Prerequisites

Read [Day 25: Working with External Crates](../25-working-with-external-crates/README.md) and [Day 20: File Handling](../20-file-handling/README.md).

## Configuration sources

Configuration can come from command-line arguments, environment variables, configuration files, or defaults in code. A simple pattern is to load non-secret defaults from a file and let environment variables override them. Choose a configuration format that suits the application; TOML, JSON, and YAML are common formats with crate support.

`std::env::var` reads a Unicode environment variable and returns a `Result<String, VarError>`:

```rust
let mode = std::env::var("APP_MODE").unwrap_or_else(|_| String::from("development"));
```

For optional settings, use `env::var(...).ok()`. Avoid assuming a variable exists.

## Keep secrets safe

Do not put passwords, tokens, or private keys in source files, sample configuration, command history, or logs. A local untracked file can be used during development; add it to `.gitignore`. Provide a harmless example file with placeholder values, and load real secrets from the environment or a dedicated secret manager in deployed systems.

The example checks whether `APP_TOKEN` is set but never prints its value. This helps demonstrate safe handling without requiring a real secret.

## Practical example

The example reads [`examples/app.config.example`](examples/app.config.example), then lets environment variables override those settings. Run it from the repository root:

```sh
rustc days/26-environment-and-configuration/examples/config.rs -o config
./config
```

By default, expected output is:

```text
Application: Rust Learning App
Mode: development
Token configured: no
```

Set `APP_MODE=production` to override the mode, and set `APP_TOKEN` to indicate a token is available. The program only prints whether a token exists, never its contents.

## Common mistakes

- **Assuming an environment variable is set:** Use a default or report a clear configuration error.
- **Printing secrets while debugging:** Log presence or a redacted value only; never dump the token.
- **Committing local configuration:** Ignore local secret files and commit only safe example templates.
- **Hard-coding deployment-specific values:** Read settings from configuration so environments can differ.
- **Treating every string setting as valid:** Parse and validate ports, URLs, and other structured values before using them.

## Exercises

Try these before reading the [Day 26 solutions](../../solutions/26-environment-and-configuration.md).

1. Add `APP_REGION` with a default value and print it.
2. Override the mode in the shell and observe the result.
3. Add a numeric port setting and handle invalid input with `Result`.
4. Create a local `.env`-style file, add it to `.gitignore`, and keep real secrets out of it when sharing the project.
5. Explain why a committed example config should use placeholders rather than real credentials.

## Further reading

- [The Twelve-Factor App: Config](https://12factor.net/config)
- [The Rust Book: Environment Variables](https://doc.rust-lang.org/book/ch12-05-working-with-environment-variables.html)
- [Cargo Book: Configuration](https://doc.rust-lang.org/cargo/reference/config.html)

## Course navigation

- [Course index](../../README.md)
- Previous: [Day 25: Working with External Crates](../25-working-with-external-crates/README.md)
- Next: [Day 27: Command-Line Applications](../27-command-line-applications/README.md)
