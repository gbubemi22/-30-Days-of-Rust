# Day 3: Data Types — solutions

Try the [exercises](../days/03-data-types/README.md#exercises) first. These are example answers; equivalent types or wording may also be correct.

## 1. A temperature that can be negative

```rust
let temperature_c: i32 = -4;
println!("Temperature: {temperature_c} °C");
```

`i32` is signed, so it can represent negative values.

## 2. Price, stock status, and marker

```rust
let price: f64 = 4.50;
let in_stock: bool = true;
let marker: char = 'B';
println!("Price: {price}; in stock: {in_stock}; category: {marker}");
```

## 3. A tuple for a book

```rust
let book = ("The Rust Book", 560);
let (title, pages) = book;
println!("{title} has {pages} pages.");
```

The compiler infers a string slice and an integer for the tuple elements.

## 4. An array of step counts

```rust
let steps = [1200, 2400, 1800, 3100];
println!("First day: {}; last day: {}", steps[0], steps[3]);
```

The array has four elements, so its last index is `3`.

## 5. Values with different numeric types

```rust
let readings = [18.0_f64, 19.5, 20.0];
```

An array must contain one element type. Writing the first value as `18.0_f64` makes all three values floating-point numbers while preserving their numeric values.
