# Day 15: Lifetimes

A reference is only safe while the value it points to is still alive. Rust checks this relationship and calls it a **lifetime**. Most of the time the compiler infers lifetimes for you; explicit annotations are needed when a function's signature must describe how input references relate to its output.

## Learning objectives

- explain what a lifetime describes;
- recognize why returning a reference to a local value is invalid;
- read lifetime annotations in function signatures;
- use a shared lifetime parameter for related input and output references;
- understand that annotations describe relationships rather than keeping data alive.

## Prerequisites

Read [Day 14: Traits](../14-traits/README.md) and review [Day 7: References and Borrowing](../07-references-and-borrowing/README.md). You should understand references and scopes.

## Why lifetimes?

Consider a function that returns one of two string references. Rust needs to know how long the returned reference is valid relative to its inputs. A lifetime annotation names that relationship; it does not extend the life of either input.

This function cannot return a reference to its local `String`:

```rust
// Invalid: `text` is dropped when the function returns.
// fn dangling() -> &str {
//     let text = String::from("temporary");
//     &text
// }
```

The caller would receive a reference to data that no longer exists. Return an owned `String`, or return a reference to input data that remains alive.

## Lifetime annotations

Lifetime names start with an apostrophe, such as `'a`. They are written in a function signature:

```rust
fn first<'a>(left: &'a str, right: &'a str) -> &'a str {
    if left.len() >= right.len() { left } else { right }
}
```

This says the returned slice is connected to the input references and cannot be used longer than the shorter-lived input. The body returns one of those inputs, so the relationship is accurate. `'a` is a name chosen for the relationship, not a fixed duration.

When a function has one input reference and returns a reference, Rust can often infer the relationship without writing an annotation. In the example above there are two input references, so the signature needs to state how the output relates to them.

## Lifetimes on structs

A struct that stores a reference must name the lifetime so Rust can ensure the referenced data lives long enough:

```rust
struct Excerpt<'a> {
    text: &'a str,
}
```

An `Excerpt<'a>` cannot outlive the text it borrows. For many beginner programs, storing an owned `String` in the struct is simpler; use a borrowed field when sharing existing text is useful.

## Practical example

Open [`examples/longer_text.rs`](examples/longer_text.rs):

```rust
fn longer<'a>(first: &'a str, second: &'a str) -> &'a str {
    if first.len() >= second.len() { first } else { second }
}

fn main() {
    let first = String::from("borrowed text");
    let second = String::from("Rust");
    let result = longer(&first, &second);
    println!("Longer text: {result}");
}
```

Compile and run from the repository root:

```sh
rustc days/15-lifetimes/examples/longer_text.rs -o longer_text
./longer_text
```

Expected output:

```text
Longer text: borrowed text
```

The function returns one of its input slices. The shared `'a` annotation tells the compiler that the result is valid only as long as the input references it may return. Both owned strings remain in scope while the result is used.

## Common mistakes

- **Treating a lifetime as a way to keep data alive:** An annotation describes a valid relationship; it cannot extend a value's scope.
- **Returning a reference to a local variable:** Local data is dropped when the function returns. Return owned data or borrow from an input.
- **Adding annotations everywhere:** Rust infers many lifetimes. Write them when needed to make relationships clear to the compiler.
- **Assuming two references always have the same lifetime:** A function may receive references with different lifetimes; a shared lifetime on inputs means the returned reference is constrained by both.
- **Thinking `'a` is a special fixed lifetime:** It is a placeholder name; `'a` is simply the common convention.

## Exercises

Try these before reading the [Day 15 solutions](../../solutions/15-lifetimes.md).

1. Add lifetime annotations to a function that returns the first of two `&str` inputs.
2. Write a function that returns the longer of two string slices and call it with two string literals.
3. Define a `Quote<'a>` struct that stores a borrowed `&'a str` and create one from a string that outlives it.
4. Explain why an annotation cannot make a reference to a local variable valid after the function ends.
5. Read `fn choose<'a>(x: &'a str, y: &str) -> &'a str`. Which input can the returned reference point to?

## Further reading

- [The Rust Programming Language: Validating References with Lifetimes](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html)
- [Rust By Example: Lifetimes](https://doc.rust-lang.org/rust-by-example/scope/lifetime.html)

## Course navigation

- [Course index](../../README.md)
- Previous: [Day 14: Traits](../14-traits/README.md)
- Next: [Day 16: Modules and Packages](../16-modules-and-packages/README.md)
