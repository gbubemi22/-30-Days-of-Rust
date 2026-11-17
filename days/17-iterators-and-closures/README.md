# Day 17: Iterators and Closures

An iterator produces a sequence of values one at a time. Closures are small functions that can be stored in variables or passed to other functions. Together they make it easy to describe transformations and filters on collections without writing every loop by hand.

## Learning objectives

- create closures and understand their parameter syntax;
- get an iterator from a collection;
- distinguish iterator adapters from consuming methods;
- use `map`, `filter`, and `collect`;
- collect a sequence of `Result` values into one `Result`.

## Prerequisites

Read [Day 16: Modules and Packages](../16-modules-and-packages/README.md) and review [Day 11: Collections](../11-collections/README.md).

## Closures

A closure is an anonymous function expression. It uses `||` around its parameters:

```rust
let add_one = |number: i32| number + 1;
println!("{}", add_one(4));
```

Rust often infers closure parameter and return types from how the closure is used. A closure can also use values from the surrounding scope:

```rust
let threshold = 10;
let is_large = |value: &i32| *value > threshold;
```

Closures can capture values from their environment. The details of how they borrow or take ownership depend on how the captured value is used; for now, observe whether the outer value remains usable.

## Iterators and adapters

Collections can produce iterators. For example, `.iter()` borrows each element, `.iter_mut()` allows changing elements, and `.into_iter()` consumes the collection and yields its elements. An adapter transforms one iterator into another:

```rust
let numbers = vec![1, 2, 3, 4];
let doubled = numbers.iter().map(|number| number * 2);
```

Adapters such as `map` and `filter` are **lazy**: they describe work, but do not do it until a consuming method asks for values. Common consuming methods include `collect`, `sum`, `count`, `find`, and `for_each`.

```rust
let doubled: Vec<i32> = numbers.iter().map(|number| number * 2).collect();
```

`collect` can build many kinds of collections, so a type annotation often tells Rust what result you want.

## Collecting results

If an iterator yields `Result<T, E>`, collecting into `Result<Vec<T>, E>` succeeds only if every item is `Ok`; the first `Err` becomes the result:

```rust
let parsed: Result<Vec<i32>, _> = ["3", "8"].iter().map(|text| text.parse()).collect();
```

This can make a pipeline concise while preserving errors.

## Practical example

Open [`examples/temperatures.rs`](examples/temperatures.rs):

```rust
fn main() {
    let readings = vec![12, 18, 21, 9, 25];
    let comfortable: Vec<i32> = readings
        .iter()
        .copied()
        .filter(|temperature| *temperature >= 18)
        .map(|temperature| temperature + 1)
        .collect();

    println!("Adjusted comfortable readings: {comfortable:?}");

    let labels: Vec<String> = readings
        .iter()
        .map(|temperature| format!("{temperature}°C"))
        .collect();
    println!("All readings: {labels:?}");
}
```

Compile and run from the repository root:

```sh
rustc days/17-iterators-and-closures/examples/temperatures.rs -o temperatures
./temperatures
```

Expected output:

```text
Adjusted comfortable readings: [19, 22, 26]
All readings: ["12°C", "18°C", "21°C", "9°C", "25°C"]
```

The first pipeline copies each integer, keeps values at least 18, adds one, and collects the results into a new vector. The second pipeline turns each reading into a text label. The original vector remains available because `.iter()` borrows it.

## Common mistakes

- **Expecting an adapter to run immediately:** `map` and `filter` are lazy. Call a consuming method such as `collect` or `sum` to make them run.
- **Forgetting the output type for `collect`:** Add a type annotation like `let values: Vec<_> = ...collect();` when inference is ambiguous.
- **Confusing `.iter()` and `.into_iter()`:** `.iter()` borrows items; `.into_iter()` consumes the collection when used by value.
- **Using the wrong reference level in a filter closure:** `filter` predicates receive a reference to an iterator item. Inspect the types or dereference where needed.
- **Writing a loop when a clear pipeline is simpler (or vice versa):** Use the form that makes the steps easiest to understand.

## Exercises

Try these before reading the [Day 17 solutions](../../solutions/17-iterators-and-closures.md).

1. Write a closure that multiplies a number by three and call it.
2. Given a vector of integers, use an iterator to make a new vector containing their squares.
3. Filter a list of words to keep only those longer than four characters.
4. Parse a list of numeric strings into `Result<Vec<i32>, _>` with `map` and `collect`.
5. Explain why the original vector remains usable after `.iter().map(...)`.

## Further reading

- [The Rust Programming Language: Closures](https://doc.rust-lang.org/book/ch13-01-closures.html)
- [The Rust Programming Language: Iterators](https://doc.rust-lang.org/book/ch13-02-iterators.html)
- [Rust By Example: Iterators](https://doc.rust-lang.org/rust-by-example/trait/iter.html)

## Course navigation

- [Course index](../../README.md)
- Previous: [Day 16: Modules and Packages](../16-modules-and-packages/README.md)
- Next: [Day 18: Smart Pointers](../18-smart-pointers/README.md)
