# Day 8: Strings and Slices

Rust has two commonly used text types: `String`, which owns growable text, and `&str`, a borrowed view of text. Slices also let you borrow part of a string or array without copying the whole value. Today you will learn how to create, inspect, change, and safely borrow text and other sequences.

## Learning objectives

- distinguish an owned `String` from a borrowed `&str`;
- create and change strings with common methods;
- pass text to functions using `&str`;
- create string and array slices;
- understand why string indexes use byte offsets and must be character boundaries.

## Prerequisites

Read [Day 7: References and Borrowing](../07-references-and-borrowing/README.md). You should understand ownership and references.

## `String` and `&str`

`String` owns text that can grow or shrink. You can create one from a string literal with `String::from` or `.to_string()`:

```rust
let mut title = String::from("Rust");
title.push_str(" basics");
title.push('!');
```

`&str`, pronounced “string slice,” is a reference to text. A string literal such as `"hello"` has type `&'static str`; you can also borrow a view from a `String`:

```rust
let owned = String::from("hello");
let borrowed: &str = &owned;
```

The owned `String` can be changed when it is mutable. A `&str` is a read-only view, so it does not let you change the text. Functions that only need to read text often accept `&str`, which works with both literals and borrowed `String` values:

```rust
fn greet(name: &str) {
    println!("Hello, {name}!");
}
```

## Common string operations

```rust
let mut text = String::from("Rust");
text.push('!');                  // add one char
text.push_str(" is fun");        // append a string slice
let combined = format!("{text}!"); // create a new String
```

`push` appends a `char`, while `push_str` appends an `&str`. `format!` builds a new `String` without taking ownership of the values interpolated into it. The `+` operator can also concatenate strings, but it takes ownership of its left-hand `String`; `format!` is often easier for beginners to read.

`len()` reports the number of bytes, not necessarily the number of visible characters. For example, many emoji use more than one byte. `chars().count()` counts Unicode scalar values, though a displayed symbol can itself contain multiple scalar values.

## String slices

A string slice borrows a range of bytes from text. The range endpoints use byte offsets:

```rust
let word = String::from("hello");
let first_part = &word[0..2]; // "he"
```

The start is included and the end is excluded, so `0..2` includes byte positions 0 and 1. `&word[..2]` starts from the beginning; `&word[2..]` continues to the end; `&word[..]` borrows the whole string.

String slices must start and end at UTF-8 character boundaries. A range that cuts through the bytes of a character will panic at runtime. For this reason, avoid guessing byte indexes when handling arbitrary text. Use methods such as `chars`, `char_indices`, or `split_whitespace` when the operation is about characters or words.

## Slices of arrays and other sequences

Slices work with arrays too. `&[T]` is a borrowed view of a sequence of values of type `T`:

```rust
let scores = [10, 20, 30, 40];
let middle: &[i32] = &scores[1..3];
```

The slice borrows elements 20 and 30; it does not make a new array. A function can accept `&[i32]` to work with arrays of different lengths and other compatible sequences:

```rust
fn total(values: &[i32]) -> i32 {
    values.iter().sum()
}
```

## Practical example

Open [`examples/text_report.rs`](examples/text_report.rs):

```rust
fn first_word(text: &str) -> &str {
    text.split_whitespace().next().unwrap_or("")
}

fn main() {
    let mut note = String::from("Rust makes careful programs");
    let greeting = format!("Study note: {note}");
    let word = first_word(&note);
    let scores = [8, 9, 10, 7];
    let recent_scores = &scores[1..3];

    println!("{greeting}");
    println!("First word: {word}");
    println!("Recent scores: {recent_scores:?}");

    note.push('!');
    println!("Updated note: {note}");
}
```

Compile and run from the repository root:

```sh
rustc days/08-strings-and-slices/examples/text_report.rs -o text_report
./text_report
```

Expected output:

```text
Study note: Rust makes careful programs
First word: Rust
Recent scores: [9, 10]
Updated note: Rust makes careful programs!
```

`first_word` accepts a borrowed `&str` and returns a slice into that text. `greeting` is a separately owned string created by `format!`. The array slice borrows only the middle two scores. The immutable borrow used for `word` is no longer needed before `note` is changed.

## Common mistakes

- **Expecting `String` and `&str` to be interchangeable in every context:** A `String` owns text; `&str` borrows text. A `&String` can usually be passed where `&str` is expected through deref coercion.
- **Trying to change text through `&str`:** A string slice is read-only. Use a mutable `String` when you need to append or modify text.
- **Treating `len()` as a character count:** `len()` counts bytes. Use `chars().count()` for Unicode scalar values.
- **Slicing at an arbitrary string index:** String ranges are byte ranges and must align with UTF-8 character boundaries. Prefer text-aware methods for arbitrary input.
- **Using a slice after changing the borrowed value:** An active immutable slice prevents conflicting mutation. Finish using the slice before changing its owner.
- **Assuming a slice is a copy:** A slice is a view borrowing the original sequence.

## Exercises

Try these before reading the [Day 8 solutions](../../solutions/08-strings-and-slices.md).

1. Create a mutable `String` with your favorite subject, append `" is interesting"`, and print it.
2. Write a function that accepts `&str` and returns the number of whitespace-separated words. Call it with a literal and a `String`.
3. Borrow the first two elements of a five-element integer array as a slice and print the slice.
4. Given the ASCII string `"ownership"`, create a slice containing `"own"`.
5. Print both `.len()` and `.chars().count()` for a string containing an emoji. Explain why they may differ.

## Further reading

- [The Rust Programming Language: Strings](https://doc.rust-lang.org/book/ch08-02-strings.html)
- [Rust By Example: Slices](https://doc.rust-lang.org/rust-by-example/scope/borrow/slice.html)
- [Standard library: `String`](https://doc.rust-lang.org/std/string/struct.String.html)

## Course navigation

- [Course index](../../README.md)
- Previous: [Day 7: References and Borrowing](../07-references-and-borrowing/README.md)
- Next: [Day 9: Structs and Methods](../09-structs-and-methods/README.md)
