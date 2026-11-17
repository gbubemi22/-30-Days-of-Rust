# Day 3: Data Types

Every Rust value has a type. Types describe what kind of value it is and which operations make sense for it. Today you will meet numbers, booleans, characters, tuples, and arrays, and see how Rust infers types or accepts explicit annotations.

## Learning objectives

- identify common integer and floating-point types;
- use booleans and Unicode `char` values;
- create and access tuples and fixed-size arrays;
- distinguish type inference from a type annotation;
- recognize a few common type-related compiler errors.

## Prerequisites

Read [Day 2: Variables and Mutability](../02-variables-and-mutability/README.md). You should know how to declare a variable and run a Rust example with `rustc`.

## Type inference and annotations

Rust is **statically typed**: the compiler determines the type of every value before the program runs. You usually do not need to write every type because Rust can infer one from how a value is used:

```rust
let count = 12; // inferred as i32 in this context
```

You can also write an annotation with a colon:

```rust
let count: u32 = 12;
```

An annotation is helpful when a value could reasonably have more than one type or when the type makes the code clearer. If Rust cannot infer enough information, it reports that it needs a type annotation.

## Integer numbers

Integers are whole numbers. Rust provides signed types (which can be negative) and unsigned types (which cannot be negative). The number in the type name is its bit width:

| Signed | Unsigned | Width |
| --- | --- | --- |
| `i8` | `u8` | 8 bits |
| `i16` | `u16` | 16 bits |
| `i32` | `u32` | 32 bits |
| `i64` | `u64` | 64 bits |
| `i128` | `u128` | 128 bits |
| `isize` | `usize` | Platform-dependent |

An `i8` can represent negative and positive values; a `u8` ranges from zero upward. The wider the type, the larger the values it can represent. Unsuffixed integer literals default to `i32` when the surrounding code does not determine another integer type. `usize` is commonly used for collection indexes and lengths.

Rust also supports readable numeric separators such as `1_000`. Choose a type that fits the values and operations you need; do not use unsigned types merely because a value is usually positive if negative values could make sense.

## Floating-point numbers

Floating-point types represent numbers with a fractional part. Rust has `f32` and `f64`; `f64` is the default for decimal literals and is often a good general choice.

```rust
let temperature: f64 = 21.5;
let ratio: f32 = 0.75;
```

Floating-point calculations can have small rounding differences because many decimal fractions cannot be represented exactly in binary. Avoid comparing calculated decimal values for exact equality in programs where precision matters.

## Booleans and characters

`bool` has two values: `true` and `false`. It is useful for yes/no conditions:

```rust
let is_ready: bool = true;
```

`char` stores one Unicode scalar value and is written with single quotes:

```rust
let initial: char = 'R';
let symbol: char = '🦀';
```

A `char` is not the same as a string: it holds one character value, while a string can contain multiple characters. Some visible symbols are made from more than one Unicode scalar value, so a `char` does not always correspond to one user-perceived symbol.

## Tuples

A tuple groups a fixed number of values, which can have different types:

```rust
let point: (i32, i32) = (4, 9);
let (x, y) = point;
println!("x={x}, y={y}");
```

You can pull values out by destructuring, as above, or by using a dot and a zero-based position such as `point.0` and `point.1`. Tuple positions are fixed by the tuple's type.

## Arrays

An array contains a fixed number of values, and all elements have the same type:

```rust
let weekdays: [&str; 3] = ["Monday", "Tuesday", "Wednesday"];
println!("{}", weekdays[0]);
```

The type `[&str; 3]` means an array of three string slices. Array indexes start at zero, so the first item is index `0`. Accessing an index outside the array's range causes a runtime panic; Rust checks the index so it does not silently read unrelated memory. Collections that can grow or shrink are covered on Day 11.

## Practical example

Open [`examples/data_types.rs`](examples/data_types.rs):

```rust
fn main() {
    let distance_km: f64 = 12.5;
    let stops: u32 = 3;
    let is_open = true;
    let symbol = 'R';

    let location: (&str, i32, i32) = ("Park", 4, 9);
    let (place, x, y) = location;
    let route = ["Park", "Museum", "Library"];

    println!("Distance: {distance_km} km across {stops} stops");
    println!("Open: {is_open}; marker: {symbol}");
    println!("{place} is at ({x}, {y})");
    println!("Route starts at {} and ends at {}", route[0], route[2]);
}
```

Compile and run from the repository root:

```sh
rustc days/03-data-types/examples/data_types.rs -o data_types
./data_types
```

Expected output:

```text
Distance: 12.5 km across 3 stops
Open: true; marker: R
Park is at (4, 9)
Route starts at Park and ends at Library
```

`distance_km` and `stops` have written annotations. Rust infers `is_open` as `bool` from `true` and `symbol` as `char` from the single-quoted value. `location` is a tuple holding a string slice and two integers; destructuring gives each part a name. `route` is an array of three string slices, and the print statement indexes its first and last elements.

## Common mistakes

- **Mixing types in an array:** Every array element must have the same type. Use a tuple when the grouped values have different types.
- **Using single quotes for text:** Single quotes are for `char` values such as `'R'`; string text uses double quotes such as `"Rust"`.
- **Indexing from one:** Array indexes start at zero. For an array with three elements, valid indexes are `0`, `1`, and `2`.
- **Integer type mismatch:** Rust does not silently convert between integer types. Convert deliberately when needed; the compiler points to the operation where the types do not match.
- **Expecting an inferred type to fit a later use:** A type annotation can settle ambiguity, for example `let count: u32 = 12;`.
- **Assuming a `char` is always one visible symbol:** Some displayed emoji or combined symbols consist of multiple Unicode values.

## Exercises

Try these before reading the [Day 3 solutions](../../solutions/03-data-types.md).

1. Declare a signed integer for a temperature that may be below zero, and print it.
2. Declare a floating-point value for a price, a boolean for whether an item is in stock, and a `char` for its category marker. Print all three.
3. Create a tuple containing a book title and its page count. Destructure it and print a sentence using both values.
4. Create an array of four daily step counts. Print the first and last values.
5. Fix this code by choosing suitable types, without changing the values: `let readings = [18, 19.5, 20];`

## Further reading

- [The Rust Programming Language: Data Types](https://doc.rust-lang.org/book/ch03-02-data-types.html)
- [Rust By Example: Primitives](https://doc.rust-lang.org/rust-by-example/primitives.html)
- [Standard library primitive types](https://doc.rust-lang.org/std/#primitives)

## Course navigation

- [Course index](../../README.md)
- Previous: [Day 2: Variables and Mutability](../02-variables-and-mutability/README.md)
- Next: [Day 4: Operators and Control Flow](../04-operators-and-control-flow/README.md)
