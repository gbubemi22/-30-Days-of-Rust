# Day 14: Traits

A trait describes behavior that different types can share. A type implements a trait by providing its required methods. Trait bounds let generic code state which behavior it needs, so the compiler can check that every accepted type provides it.

## Learning objectives

- define a trait with required methods;
- implement a trait for a type;
- use a trait bound on a generic function;
- understand that common library features such as formatting use traits;
- distinguish a trait (shared behavior) from a concrete type.

## Prerequisites

Read [Day 13: Generics](../13-generics/README.md). You should understand generic type parameters.

## Define and implement a trait

Use `trait` to describe behavior:

```rust
trait Summary { fn summary(&self) -> String; }
struct Article { title: String }

impl Summary for Article {
    fn summary(&self) -> String { format!("Article: {}", self.title) }
}
```

The implementation must provide the required methods with compatible signatures. A trait can also provide a default method body that implementors may use or override.

## Trait bounds

A generic function that needs to call `summary` can require `T: Summary`:

```rust
fn print_summary<T: Summary>(item: &T) {
    println!("{}", item.summary());
}
```

This says the function accepts any type `T` that implements `Summary`. A type without that implementation cannot be passed. Multiple bounds can be joined with `+`, for example `T: Summary + std::fmt::Display`.

## Standard library traits

Traits power many familiar operations. `Debug` enables developer-oriented output with `{:?}`; `Display` enables user-oriented output with `{}`. `Clone` creates an explicit copy, `PartialEq` enables equality comparisons, and `Iterator` describes values that can be visited in sequence. A derive attribute can implement some common traits automatically when all fields support them:

```rust
#[derive(Debug, PartialEq)]
struct Point { x: i32, y: i32 }
```

Deriving is convenient, but it still creates a real trait implementation checked by Rust.

## Practical example

Open [`examples/summaries.rs`](examples/summaries.rs). The file defines `Summary`, implements it for `Article` and `Note`, and passes both types to a generic `show` function.

Compile and run from the repository root:

```sh
rustc days/14-traits/examples/summaries.rs -o summaries
./summaries
```

Expected output:

```text
Learning Rust by A learner
Note: Practice a little each day
```

Both types implement `Summary`, so the generic `show` function can work with either. It only depends on the promised behavior, not on the concrete type's fields.

## Common mistakes

- **Writing `impl Trait for Type` in the wrong order:** Use `impl TraitName for TypeName`.
- **Calling a trait method on a type that has not implemented the trait:** Add an implementation or use a different operation.
- **Forgetting a required method:** Every required trait method must be implemented; compiler errors list missing items.
- **Assuming a trait is a value type:** A trait describes behavior; values have concrete types that may implement it.
- **Adding bounds the function does not use:** Require only the capabilities the implementation needs.

## Exercises

Try these before reading the [Day 14 solutions](../../solutions/14-traits.md).

1. Define a `Describe` trait with a method that returns a `String`.
2. Implement it for a `Book` struct and a `Song` struct.
3. Write a generic function with a `T: Describe` bound that prints the description.
4. Derive `Debug` and `PartialEq` for a simple struct and compare two instances.
5. Explain what a trait bound guarantees to a generic function.

## Further reading

- [The Rust Programming Language: Traits](https://doc.rust-lang.org/book/ch10-02-traits.html)
- [Rust By Example: Traits](https://doc.rust-lang.org/rust-by-example/trait.html)
- [Standard library: `std::fmt`](https://doc.rust-lang.org/std/fmt/)

## Course navigation

- [Course index](../../README.md)
- Previous: [Day 13: Generics](../13-generics/README.md)
- Next: [Day 15: Lifetimes](../15-lifetimes/README.md)
