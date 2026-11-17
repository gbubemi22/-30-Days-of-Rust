# Day 27: Command-Line Applications

A command-line application accepts arguments and input from a terminal, prints useful output, and returns an exit code to the operating system. Today you will read arguments, read standard input, report errors, and build a small CLI using only the standard library.

## Learning objectives

- read command-line arguments with `std::env`;
- read text from standard input;
- print normal output and error messages to the correct streams;
- return a success or failure exit code;
- distinguish a CLI's interface from its internal implementation.

## Prerequisites

Read [Day 26: Environment and Configuration](../26-environment-and-configuration/README.md) and review functions and `Result`.

## Arguments and standard input

`std::env::args()` yields the program name followed by command-line arguments. Skip the first value to process the arguments supplied by the user. If arguments may contain non-Unicode text, use `args_os()` instead.

Standard input (`stdin`) lets a program receive text from a pipe or the keyboard. `read_to_string` reads all input until end-of-file:

```rust
use std::io::{self, Read};
let mut input = String::new();
io::stdin().read_to_string(&mut input)?;
```

## Output and exit codes

Use `println!` for normal output and `eprintln!` for diagnostics. A process exit code of zero usually means success; a nonzero code signals failure to scripts and shells. `std::process::ExitCode` makes this intent explicit.

Keep command parsing separate from the work the command performs. For a larger CLI, a crate such as `clap` can parse flags and generate help text; for a small tool, standard-library argument handling may be enough.

## Practical example

Open [`examples/word-tool.rs`](examples/word-tool.rs). It has a `greet` subcommand and a `count` subcommand that reads standard input:

```sh
rustc days/27-command-line-applications/examples/word-tool.rs -o word-tool
./word-tool greet Ada
printf 'Rust makes systems programming approachable' | ./word-tool count
```

Expected output:

```text
Hello, Ada!
Words: 5
```

Try omitting a name or using an unknown command to see an error message and a nonzero exit code. In PowerShell, pipe text to `.\word-tool.exe count` to test standard input.

## Common mistakes

- **Forgetting that `args()` includes the executable name:** Call `.skip(1)` when reading user arguments.
- **Writing diagnostics to stdout:** Use `eprintln!` for errors so normal output can still be piped.
- **Returning success after invalid input:** Use a nonzero exit code when the command cannot complete as requested.
- **Assuming arguments are always present:** Check for missing values and show a useful usage message.
- **Reading only one line when the command accepts piped text:** Use a loop or `read_to_string` as appropriate.
- **Using `args()` for arbitrary operating-system strings:** Use `args_os()` when non-Unicode arguments must be supported.

## Exercises

Try these before reading the [Day 27 solutions](../../solutions/27-command-line-applications.md).

1. Add a `chars` subcommand that counts Unicode scalar values from standard input.
2. Add an optional name to `greet` and show a default greeting when omitted.
3. Make the `count` output include both words and lines.
4. Run an invalid command and inspect its exit code in your shell.
5. Explain the difference between standard output and standard error.

## Further reading

- [The Rust Programming Language: Accepting Command-Line Arguments](https://doc.rust-lang.org/book/ch12-01-accepting-command-line-arguments.html)
- [Standard library: `std::env`](https://doc.rust-lang.org/std/env/index.html)
- [Standard library: `std::process::ExitCode`](https://doc.rust-lang.org/std/process/struct.ExitCode.html)

## Course navigation

- [Course index](../../README.md)
- Previous: [Day 26: Environment and Configuration](../26-environment-and-configuration/README.md)
- Next: [Day 28: Code Quality and Documentation](../28-code-quality-and-documentation/README.md)
