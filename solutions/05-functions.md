# Day 5: Functions — solutions

Try the [exercises](../days/05-functions/README.md#exercises) first. These are sample solutions; equivalent names or output wording are fine.

## 1. Print a welcome message

```rust
fn print_welcome() {
    println!("Welcome to the program!");
}

fn main() {
    print_welcome();
}
```

## 2. Return a square

```rust
fn square(number: i32) -> i32 {
    number * number
}

fn main() {
    println!("{}", square(5));
}
```

The final expression is returned because it has no semicolon.

## 3. Check whether an integer is even

```rust
fn is_even(number: i32) -> bool {
    number % 2 == 0
}
```

The comparison evaluates to a boolean, matching the declared return type.

## 4. Describe a book

```rust
fn describe_book(title: &str, pages: u32) {
    println!("{title} has {pages} pages.");
}
```

## 5. Return the average

```rust
fn average(first: f64, second: f64) -> f64 {
    (first + second) / 2.0
}

fn main() {
    println!("Average: {}", average(8.0, 10.0));
}
```

The expected average is `9` (display formatting may show `9` or `9.0`).

## 6. Observe the semicolon

Without the semicolon, the final expression evaluates to its value and can be returned. With a semicolon, it becomes a statement and evaluates to `()`. If the function promises an `i32`, the compiler then reports a mismatch because it found `()` instead.
