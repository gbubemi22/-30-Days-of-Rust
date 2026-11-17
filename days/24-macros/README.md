# Day 24: Macros

Macros generate Rust code from Rust syntax. You have already used macros such as `println!`, `vec!`, and `format!`. Today you will write a small declarative macro with `macro_rules!`, understand common built-in macros, and learn what procedural macros do.

## Learning objectives

- recognize macro invocation syntax and why macros use `!`;
- define a simple declarative macro with `macro_rules!`;
- match macro inputs and expand them into code;
- name common standard macros;
- describe procedural macros at a high level.

## Prerequisites

Read [Day 23: Async Rust Fundamentals](../23-async-rust-fundamentals/README.md). You should know functions, expressions, and basic Rust syntax.

## Why macros?

A function accepts values at runtime and follows a function signature. A macro receives syntax and generates code during compilation. Macros can accept a varying number of arguments or generate repeated structures that would be awkward to express with a function alone.

The `!` distinguishes macro calls such as `println!(...)` from ordinary function calls. Common standard macros include `format!`, `vec!`, `assert_eq!`, `panic!`, and `matches!`.

## Declarative macros with `macro_rules!`

`macro_rules!` macros match input patterns and expand to code:

```rust
macro_rules! announce {
    ($message:expr) => {
        println!("Announcement: {}", $message);
    };
}

announce!("The lesson is ready");
```

`$message:expr` means the macro accepts one Rust expression. The `=>` separates the input pattern from the code to generate. Parentheses, braces, and brackets can all delimit a macro invocation.

Macros can match repeated inputs with `$(...),*` or `$(...);+` patterns. Repetition is powerful, but start with a small macro and prefer a normal function when the behavior is ordinary runtime logic.

## Procedural macros

Procedural macros operate on Rust token streams and generate Rust code. They are commonly written in a separate crate with `proc-macro = true`. Three forms are:

- **Derive macros**, used with `#[derive(...)]` to implement traits for a type.
- **Attribute macros**, which attach to an item and transform it, such as `#[some_attribute]`.
- **Function-like procedural macros**, invoked with `name!(...)` and given custom input syntax.

You have already used derive macros such as `#[derive(Debug)]` and Serde's `#[derive(Serialize)]`. Writing procedural macros requires more advanced Rust and is outside this beginner example.

## Practical example

Open [`examples/announce.rs`](examples/announce.rs):

```rust
macro_rules! announce_many {
    ($($message:expr),+ $(,)?) => {
        $(println!("Announcement: {}", $message);)+
    };
}

fn main() {
    announce_many!("Practice", "Read compiler messages", "Keep going");
}
```

Compile and run from the repository root:

```sh
rustc days/24-macros/examples/announce.rs -o announce
./announce
```

Expected output:

```text
Announcement: Practice
Announcement: Read compiler messages
Announcement: Keep going
```

`$($message:expr),+` matches one or more comma-separated expressions. The matching repetition in the expansion creates one `println!` call for each expression. `$(,)?` permits an optional trailing comma.

## Common mistakes

- **Calling a macro like a function:** Macro invocations need `!`, such as `announce!("hello")`.
- **Using the wrong fragment specifier:** `expr` matches an expression; other specifiers match patterns, types, identifiers, and other syntax.
- **Forgetting repetition syntax:** Repeated input bindings must also be repeated in the expansion.
- **Writing a macro for ordinary logic:** Functions are usually easier to read, type-check, and debug for normal runtime behavior.
- **Assuming derive macros are built into the language:** Some derives are standard-library macros; others, such as Serde derives, come from dependencies.

## Exercises

Try these before reading the [Day 24 solutions](../../solutions/24-macros.md).

1. Write a macro `say_hi!` that prints a fixed greeting.
2. Change `announce_many!` to accept semicolon-separated expressions.
3. Use `matches!` to check whether a value is a particular enum variant.
4. Find where `println!` or `vec!` is used in an earlier lesson and explain why a macro is useful there.
5. In your own words, distinguish declarative and procedural macros.

## Further reading

- [The Rust Programming Language: Macros](https://doc.rust-lang.org/book/ch19-06-macros.html)
- [Rust By Example: `macro_rules!`](https://doc.rust-lang.org/rust-by-example/macros.html)
- [The Rust Reference: Macros](https://doc.rust-lang.org/reference/macros.html)

## Course navigation

- [Course index](../../README.md)
- Previous: [Day 23: Async Rust Fundamentals](../23-async-rust-fundamentals/README.md)
- Next: [Day 25: Working with External Crates](../25-working-with-external-crates/README.md)
