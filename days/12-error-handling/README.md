# Day 12: Error Handling

Programs encounter situations that are not part of the happy path: a search may find nothing, a file may be missing, or input may not be a number. Rust represents expected failures in types so callers must decide how to handle them. Today you will use `Option`, `Result`, the `?` operator, and `panic!` appropriately.

## Learning objectives

- distinguish `Option<T>` from `Result<T, E>`;
- handle both variants with `match`;
- propagate errors with `?`;
- return useful errors from functions;
- distinguish recoverable errors from bugs or unrecoverable states.

## Prerequisites

Read [Day 11: Collections](../11-collections/README.md). You should know enums, pattern matching, and how `Option` represents a missing value.

## `Option<T>`: a value may be absent

`Option<T>` is either `Some(value)` or `None`. For example, a vector lookup returns an `Option` because the requested position might not exist:

```rust
let names = vec!["Ada", "Linus"];
match names.get(4) {
    Some(name) => println!("Found {name}"),
    None => println!("No name at that position"),
}
```

Use `Option` when absence is expected and there is no error detail to report.

## `Result<T, E>`: an operation may fail

`Result<T, E>` is either `Ok(value)` or `Err(error)`. Parsing text as a number can fail, so `parse` returns a `Result`:

```rust
let parsed = "42".parse::<i32>();
match parsed {
    Ok(number) => println!("Parsed {number}"),
    Err(error) => println!("Could not parse number: {error}"),
}
```

The `T` is the success value type; `E` is the error type. Handle the error, convert it into a message, or pass it to a caller that can decide what to do.

## Propagate errors with `?`

Inside a function that returns a compatible `Result`, `?` unwraps an `Ok` value or returns early with the `Err` value:

```rust
fn parse_count(text: &str) -> Result<u32, std::num::ParseIntError> {
    let count = text.parse::<u32>()?;
    Ok(count)
}
```

The `?` does not ignore an error. It passes it to the caller. The caller still chooses how to report or recover from it. `?` also works with `Option` inside a function returning `Option`.

## Recoverable and unrecoverable errors

A recoverable error is one the program can respond to, such as invalid user input or a missing optional file. `Result` is the normal tool for these cases. An unrecoverable error usually indicates a bug or a condition from which the program cannot sensibly continue; `panic!` stops execution and prints diagnostic information.

Do not use `panic!` for ordinary input problems. Likewise, avoid `.unwrap()` or `.expect()` when an error should be handled gracefully. They are useful in small demonstrations, prototypes, or when a failure truly means the program cannot proceed.

## Practical example

Open [`examples/parse_score.rs`](examples/parse_score.rs):

```rust
fn parse_score(text: &str) -> Result<u32, std::num::ParseIntError> {
    let score = text.trim().parse::<u32>()?;
    Ok(score)
}

fn main() {
    for input in ["87", "not a score"] {
        match parse_score(input) {
            Ok(score) => println!("Score: {score}"),
            Err(error) => println!("Invalid score '{input}': {error}"),
        }
    }
}
```

Run from the repository root:

```sh
rustc days/12-error-handling/examples/parse_score.rs -o parse_score
./parse_score
```

Expected output:

```text
Score: 87
Invalid score 'not a score': invalid digit found in string
```

`parse_score` trims whitespace and uses `?` to return a parsing error to its caller. `main` handles both outcomes with `match`, so invalid input is reported without crashing.

## Common mistakes

- **Ignoring an `Option` or `Result`:** Match on it, use a deliberate fallback, or propagate it with `?` from a compatible function.
- **Using `?` in a function with the wrong return type:** `?` needs a function that can return the same kind of failure, or a compatible converted error.
- **Unwrapping untrusted input:** `.unwrap()` panics on an error. Handle input errors when the program can continue.
- **Using `panic!` for expected failures:** Invalid input and missing resources are usually recoverable and belong in `Result`.
- **Returning only success from a fallible function:** Declare `Result<T, E>` and return `Ok(value)` or an error.

## Exercises

Try these before reading the [Day 12 solutions](../../solutions/12-error-handling.md).

1. Write a function that returns the first element of a slice as `Option<&i32>`.
2. Write `parse_age` that returns `Result<u8, _>` when parsing text as an age.
3. Handle both successful and failed results with `match`, printing a clear message.
4. Create a function that parses two numbers with `?` and returns their sum as `Result<i32, ParseIntError>`.
5. Explain when `Option` is a better fit than `Result`, and when `Result` is needed.

## Further reading

- [The Rust Programming Language: Error Handling](https://doc.rust-lang.org/book/ch09-00-error-handling.html)
- [Rust By Example: `Result`](https://doc.rust-lang.org/rust-by-example/error/result.html)
- [Standard library: `Result`](https://doc.rust-lang.org/std/result/enum.Result.html)

## Course navigation

- [Course index](../../README.md)
- Previous: [Day 11: Collections](../11-collections/README.md)
- Next: [Day 13: Generics](../13-generics/README.md)
