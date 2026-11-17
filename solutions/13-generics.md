# Day 13: Generics — solutions

Try the [exercises](../days/13-generics/README.md#exercises) first. These are sample definitions; names may differ.

## 1. Generic identity

```rust
fn identity<T>(value: T) -> T { value }
let enabled = identity(true);
let label = identity(String::from("Rust"));
```

## 2. A triple

```rust
struct Triple<T> { first: T, second: T, third: T }
let values = Triple { first: 1, second: 2, third: 3 };
```

## 3. Coordinates with two types

```rust
struct Coordinates<X, Y> { x: X, y: Y }
let position = Coordinates { x: 4, y: 7.5 };
```

`X` and `Y` can be chosen independently.

## 4. Generic enum

```rust
enum Maybe<T> { Value(T), Empty }
let answer = Maybe::Value(42);
let nothing: Maybe<String> = Maybe::Empty;
```

The empty form needs a type annotation if no other use reveals `T`.

## 5. Why printing needs a bound

The generic type may be one that has no text representation. Printing requires the type to implement the formatting trait used by the print macro. A trait bound tells Rust that the capability exists for the types accepted by the function.
