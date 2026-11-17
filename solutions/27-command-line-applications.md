# Day 27: Command-Line Applications — solutions

Try the [exercises](../days/27-command-line-applications/README.md#exercises) first. These are example changes to the CLI.

## 1. Count Unicode scalar values

After reading `input`, print `input.chars().count()`. This counts Unicode scalar values, not grapheme clusters.

## 2. Optional greeting name

```rust
let name = args.next().unwrap_or_else(|| String::from("there"));
println!("Hello, {name}!");
```

## 3. Count words and lines

```rust
println!("Words: {}", input.split_whitespace().count());
println!("Lines: {}", input.lines().count());
```

## 4. Inspect an exit code

On Unix-like shells, run the command and then inspect `$?`. On PowerShell, inspect `$LASTEXITCODE`. The invalid command should return a nonzero status.

## 5. Output streams

Standard output carries normal command results and is commonly piped to another program. Standard error carries diagnostics so those messages do not become part of a successful output stream.
