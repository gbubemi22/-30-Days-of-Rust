# Day 19: Testing and Debugging

Tests help you check that code behaves as intended and keep it working as you change it. Rust includes a test framework in the standard toolchain, and its compiler diagnostics often explain where a mistake occurred. Today you will write unit and integration tests, use assertions, and practice a simple debugging workflow.

## Learning objectives

- write unit tests using `#[test]`;
- use assertions to check expected results;
- distinguish unit tests from integration tests;
- run tests with Cargo;
- read compiler errors and use simple debugging techniques.

## Prerequisites

Read [Day 18: Smart Pointers](../18-smart-pointers/README.md). You should be familiar with functions and modules. This example uses Cargo project conventions.

## Unit tests

A unit test usually lives near the code it checks. Rust test functions use `#[test]` and are commonly placed in a `#[cfg(test)] mod tests` module:

```rust
pub fn add(left: i32, right: i32) -> i32 { left + right }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds_two_numbers() {
        assert_eq!(add(2, 3), 5);
    }
}
```

`#[cfg(test)]` includes the test module only when running tests. `use super::*` brings the parent module's items into the test module.

## Assertions

- `assert!(condition)` checks that a condition is true.
- `assert_eq!(actual, expected)` checks equality and prints both values on failure.
- `assert_ne!(left, right)` checks that two values are different.

Assertions stop that test when they fail and show the source location. A failing test is useful feedback; investigate its expected and actual values rather than removing the assertion.

## Integration tests

Integration tests exercise the public interface of a crate from outside its library module. Cargo discovers Rust files under `tests/` as separate test crates. They import the library by its package/crate name:

```rust
use testable_math::add;

#[test]
fn adds_values() {
    assert_eq!(add(4, 5), 9);
}
```

Integration tests require a Cargo package with a library target; the example project below includes one.

## Run the example tests

The example project is in [`examples/testable-math/`](examples/testable-math/). From the repository root:

```sh
cargo test --manifest-path days/19-testing-and-debugging/examples/testable-math/Cargo.toml
```

The project has a unit test beside the library function and an integration test under `tests/`. Cargo reports each test as `ok` when its assertions pass.

## Debugging workflow

When a program fails to compile or a test fails:

1. Read the first relevant error or failed assertion.
2. Note the file and line the diagnostic points to.
3. Compare the actual type or value with what the code expects.
4. Make one focused change and run the check again.

Use `dbg!(value)` for quick inspection while learning; it prints a value and its source location, then returns the value. Remove temporary debug output when it is no longer useful. For larger projects, a debugger can step through code, inspect variables, and stop at breakpoints.

## Common mistakes

- **Using `assert!` to compare values:** Prefer `assert_eq!` when equality is what you mean; it shows both sides on failure.
- **Forgetting a test attribute:** A function without `#[test]` is not run as a test.
- **Testing only the happy path:** Include boundary cases and expected failures where they matter.
- **Putting integration tests in `src/`:** Cargo's conventional integration test directory is `tests/` at the package root.
- **Ignoring the first compiler error:** Later diagnostics can be consequences of the first issue. Start with the earliest relevant one.
- **Leaving `dbg!` in user-facing output:** It is a development aid; remove or replace it when finished debugging.

## Exercises

Try these before reading the [Day 19 solutions](../../solutions/19-testing-and-debugging.md).

1. Add a test for `add` with a negative input.
2. Add a function that returns the larger of two numbers and test both argument orders.
3. Write an assertion for a boundary case, such as zero.
4. Deliberately make one expected value wrong, run the test, and read the failure report. Restore the correct expectation afterward.
5. Use `dbg!` to inspect an intermediate value and then remove it.

## Further reading

- [The Rust Programming Language: Writing Automated Tests](https://doc.rust-lang.org/book/ch11-00-testing.html)
- [Cargo Book: cargo test](https://doc.rust-lang.org/cargo/commands/cargo-test.html)
- [Rust compiler diagnostics](https://doc.rust-lang.org/rustc/diagnostics.html)

## Course navigation

- [Course index](../../README.md)
- Previous: [Day 18: Smart Pointers](../18-smart-pointers/README.md)
- Next: [Day 20: File Handling](../20-file-handling/README.md)
