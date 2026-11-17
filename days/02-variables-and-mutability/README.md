# Day 2: Variables and Mutability

Today you will learn how to give values names, when a value can change, and how Rust lets you create a new variable with the same name through shadowing.

## Learning objectives

- declare variables with `let` and make a binding mutable with `mut`;
- explain why Rust variables are immutable by default;
- declare constants and distinguish them from variables;
- use shadowing to transform a value while keeping a readable name;
- recognize the difference between changing a value and creating a new binding.

## Prerequisites

Read [Day 1: Introduction to Rust](../01-introduction/README.md). You should know how to compile and run a `.rs` file with `rustc`.

## Bindings and immutability

A variable gives a name to a value. In Rust, the usual declaration is `let`:

```rust
let score = 10;
```

Rust calls this a **binding**: the name `score` is bound to the value `10`. By default, the binding is immutable, which means you cannot assign a different value to it later:

```rust
let score = 10;
score = 11; // error: cannot assign twice to immutable variable
```

This default helps you see which parts of a program may change. When a value should change, opt in with `mut`:

```rust
let mut score = 10;
score = 11;
```

The `mut` keyword makes the binding mutable. It does not mean every value in a larger structure is automatically safe to change in every context; Rust still checks how values are used.

## Constants

A constant uses `const` and must have a type written explicitly:

```rust
const MAX_PLAYERS: u32 = 8;
```

Constants are not changed after being declared. They are useful for values that are fixed throughout a program. By convention, constant names use uppercase letters with underscores between words. A `const` can be declared outside a function, and its value must be computable at compile time.

| Declaration | Can be reassigned? | Type required? | Example use |
| --- | --- | --- | --- |
| `let` | No, unless declared `mut` | Usually inferred | A local value |
| `let mut` | Yes | Usually inferred | A counter that changes |
| `const` | No | Yes | A fixed program-wide value |

## Shadowing

You can declare a new variable using a name that is already in use in the same scope. This is called **shadowing**:

```rust
let label = "  Rust  ";
let label = label.trim();
```

The second `let` creates a new binding named `label`. It does not mutate the first binding. Shadowing can also change the value's type:

```rust
let input = "42";
let input: u32 = input.parse().expect("input should be a number");
```

Here the first `input` is text (`&str`); the second is an unsigned integer (`u32`). The first value remains immutable. A later lesson will explain the `expect` call and error handling.

## Practical example

Open [`examples/variables.rs`](examples/variables.rs):

```rust
const MINUTES_PER_HOUR: u32 = 60;

fn main() {
    let event = "Rust study";
    let mut minutes = 25;
    minutes += 5;

    let minutes = minutes.to_string();
    let message = format!("{event} lasts {minutes} minutes.");

    println!("{message}");
    println!("There are {MINUTES_PER_HOUR} minutes in an hour.");
}
```

Compile and run from the repository root:

```sh
rustc days/02-variables-and-mutability/examples/variables.rs -o variables
./variables
```

Expected output:

```text
Rust study lasts 30 minutes.
There are 60 minutes in an hour.
```

`minutes` is mutable while the number is updated. Then `let minutes = ...` shadows it with a new binding containing text. `event` and `message` never change, so they do not need `mut`. `MINUTES_PER_HOUR` is a constant with an explicit integer type. `format!` builds a `String` from the values; it does not print by itself.

## Common mistakes

- **Reassigning a plain `let`:** Add `mut` to the original binding if reassignment is intended. The compiler's “cannot assign twice” message points to the assignment.
- **Adding `mut` to a constant:** Constants cannot be mutable. Use `let mut` for a value that changes while the program runs.
- **Thinking shadowing is mutation:** A second `let` makes a new binding. This is useful when transforming a value or its type.
- **Omitting a constant's type:** `const` declarations require a type, such as `u32` or `&str`.
- **Using unclear names:** Prefer names such as `minutes_remaining` that explain what the value represents. Rust style uses `snake_case` for variable and function names, and `UPPER_SNAKE_CASE` for constants.

## Exercises

Try these before reading the [Day 2 solutions](../../solutions/02-variables-and-mutability.md).

1. Declare an immutable `city` string and print it.
2. Declare a mutable `visits` count, increase it, and print the updated count.
3. Declare a constant for the number of days in a week, with an appropriate integer type, and print it.
4. Start with a text value containing a number, parse it into a `u32` using shadowing, then print the number plus one. (The example above demonstrates the parsing pattern.)
5. In your own words, explain how `mut` differs from shadowing.

## Further reading

- [The Rust Programming Language: Variables and Mutability](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html)
- [Rust By Example: Variable Bindings](https://doc.rust-lang.org/rust-by-example/variable_bindings.html)

## Course navigation

- [Course index](../../README.md)
- Previous: [Day 1: Introduction to Rust](../01-introduction/README.md)
- Next: [Day 3: Data Types](../03-data-types/README.md)
