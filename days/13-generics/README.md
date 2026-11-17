# Day 13: Generics

Generics let you write functions and types that work with more than one concrete type while keeping type checking. Instead of duplicating similar code for `i32`, `f64`, and other types, you describe the type as a parameter. Today you will use generic functions, structs, and enums.

## Learning objectives

- read a generic type parameter such as `T`;
- write and call a generic function;
- define generic structs and enums;
- understand that generic code is still checked for each concrete type;
- recognize when a trait bound is needed (expanded on Day 14).

## Prerequisites

Read [Day 12: Error Handling](../12-error-handling/README.md). You should understand functions, structs, enums, and types.

## Generic functions

A generic function uses a type parameter in angle brackets after its name:

```rust
fn identity<T>(value: T) -> T {
    value
}

let number = identity(7);
let word = identity("hello");
```

`T` is a placeholder for a concrete type chosen when the function is called. The function takes a `T` and returns that same `T`. Rust infers the type from the argument in these calls.

Generics do not mean every operation works for every type. `identity` only returns a value, so it does not need to know what `T` can do. To compare or print generic values, the type must support those operations; trait bounds express those requirements, as you will see on Day 14.

## Generic structs

Structs can also have type parameters:

```rust
struct Pair<T> {
    first: T,
    second: T,
}

let scores = Pair { first: 8, second: 10 };
let names = Pair { first: "Ada", second: "Grace" };
```

Both fields use the same `T`. If fields need different types, the struct can have more than one parameter, such as `struct Point<T, U> { x: T, y: U }`.

## Generic enums

The standard library's `Option<T>` and `Result<T, E>` are generic enums. Their definitions can hold values of any chosen type:

```rust
enum Either<L, R> { Left(L), Right(R) }
let choice: Either<i32, &str> = Either::Right("done");
```

Here `L` and `R` can be different types. Rust often infers the parameters from the variant value; a type annotation can make the choice explicit.

## Practical example

Open [`examples/pair.rs`](examples/pair.rs):

```rust
struct Pair<T> { first: T, second: T }

impl<T> Pair<T> {
    fn new(first: T, second: T) -> Pair<T> { Pair { first, second } }
}

fn main() {
    let numbers = Pair::new(12, 30);
    let labels = Pair::new("beginner", "Rust");
    println!("{} and {}", numbers.first, numbers.second);
    println!("{} and {}", labels.first, labels.second);
}
```

Compile and run from the repository root:

```sh
rustc days/13-generics/examples/pair.rs -o pair
./pair
```

Expected output:

```text
12 and 30
beginner and Rust
```

`Pair<T>` can store two values of the same type. The `impl<T>` block says its methods are available for every `T`; `new` constructs a pair without requiring any special behavior from that type.

## Common mistakes

- **Leaving off the type parameter in an implementation:** Write `impl<T> Pair<T>` to say the methods apply to any `T`.
- **Assuming a generic type supports every operation:** A type parameter only promises that values have that type. Add a trait bound when an operation requires a capability.
- **Using different types for fields declared as the same `T`:** `Pair<T>` uses one type for both fields. Define two parameters if the fields may differ.
- **Confusing a generic parameter with a runtime value:** `T` is a type selected at compile time, not a value passed while the program runs.

## Exercises

Try these before reading the [Day 13 solutions](../../solutions/13-generics.md).

1. Write a generic `identity` function and call it with a `bool` and a `String`.
2. Define `Triple<T>` with three fields of the same type and create a value of integers.
3. Define `Coordinates<X, Y>` whose x and y fields may have different types.
4. Define a generic enum `Maybe<T>` with `Value(T)` and `Empty` variants, then create both forms.
5. Explain why a generic function that prints `T` needs an additional capability beyond merely accepting `T`.

## Further reading

- [The Rust Programming Language: Generic Types](https://doc.rust-lang.org/book/ch10-01-syntax.html)
- [Rust By Example: Generics](https://doc.rust-lang.org/rust-by-example/generics.html)

## Course navigation

- [Course index](../../README.md)
- Previous: [Day 12: Error Handling](../12-error-handling/README.md)
- Next: [Day 14: Traits](../14-traits/README.md)
