# Day 8: Strings and Slices — solutions

Try the [exercises](../days/08-strings-and-slices/README.md#exercises) first. These are example answers; equivalent code is fine.

## 1. Append to a string

```rust
let mut subject = String::from("Rust");
subject.push_str(" is interesting");
println!("{subject}");
```

## 2. Count words from a string slice

```rust
fn word_count(text: &str) -> usize {
    text.split_whitespace().count()
}

fn main() {
    let owned = String::from("Rust has useful tools");
    println!("{}", word_count("slices work with literals"));
    println!("{}", word_count(&owned));
}
```

The `&str` parameter accepts both the literal and the borrowed `String`.

## 3. Borrow the first two array elements

```rust
let values = [3, 6, 9, 12, 15];
let first_two = &values[0..2];
println!("{first_two:?}");
```

The end index is exclusive, so the slice contains indexes 0 and 1.

## 4. Slice `"own"`

```rust
let word = "ownership";
let prefix = &word[0..3];
println!("{prefix}");
```

These ASCII letters each occupy one byte, so the byte range `0..3` ends at a character boundary.

## 5. Bytes and Unicode scalar values

```rust
let symbol = "🦀";
println!("bytes: {}", symbol.len());
println!("chars: {}", symbol.chars().count());
```

The crab emoji occupies multiple UTF-8 bytes but is one Unicode scalar value, so the byte length is greater than the character count.
