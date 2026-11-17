# Day 11: Collections

Collections store groups of values. Unlike arrays, some collections can grow as your program runs. Today you will use vectors for ordered lists, hash maps for key/value pairs, and hash sets for unique values.

## Learning objectives

- create, read, update, and remove values in a `Vec<T>`;
- understand how vector indexes and `get` behave;
- store and look up key/value pairs in a `HashMap<K, V>`;
- add and check unique values with a `HashSet<T>`;
- recognize that hash map and hash set iteration order is not guaranteed.

## Prerequisites

Read [Day 10: Enums and Pattern Matching](../10-enums-and-pattern-matching/README.md). You should know about `Option`, types, and borrowing.

## Vectors: growable ordered lists

A vector (`Vec<T>`) stores values of one type in order and can grow or shrink. The type parameter `T` is the type of its elements:

```rust
let mut temperatures = vec![18, 21, 19];
temperatures.push(23);
let first = temperatures[0];
```

The `vec!` macro creates a vector containing the listed values. An empty vector can be created with `Vec::new()`, though Rust may need a type annotation if it cannot infer what it will contain:

```rust
let mut names: Vec<String> = Vec::new();
```

Use indexing (`values[index]`) when the index is known to be valid; an out-of-range index causes a panic. Use `.get(index)` when the index may be invalid. It returns an `Option<&T>`: `Some` with a reference when the element exists, or `None` otherwise.

```rust
match temperatures.get(10) {
    Some(value) => println!("Temperature: {value}"),
    None => println!("No value at that position"),
}
```

You can loop over a borrowed vector with `for value in &temperatures`. To change elements while looping, use a mutable reference with `for value in &mut temperatures`. `pop()` removes and returns the last item as an `Option<T>`.

## Hash maps: key/value pairs

A `HashMap<K, V>` associates each key with a value. Keys are unique; inserting a value for an existing key replaces its previous value:

```rust
use std::collections::HashMap;

let mut inventory = HashMap::new();
inventory.insert(String::from("apples"), 12);
inventory.insert(String::from("pears"), 8);
```

The types of the keys and values are inferred from insertion. Use `.get(key)` to look up a value; it returns an `Option<&V>` because the key might not exist:

```rust
if let Some(count) = inventory.get("apples") {
    println!("Apples: {count}");
}
```

For a counter, `entry` and `or_insert` provide a convenient way to insert a default value only when the key is missing:

```rust
let count = inventory.entry(String::from("plums")).or_insert(0);
*count += 1;
```

Hash maps do not guarantee a particular iteration order. Do not rely on the order in which their entries are printed or visited.

## Hash sets: unique values

A `HashSet<T>` stores each distinct value at most once:

```rust
use std::collections::HashSet;

let mut tags = HashSet::new();
tags.insert("rust");
tags.insert("learning");
tags.insert("rust"); // already present; set stays unique
```

`insert` returns `true` if the value was new and `false` if it was already present. Use `.contains(&value)` to check membership. Like a hash map, a hash set does not promise a stable iteration order.

Hash maps and hash sets require their keys or values to support equality and hashing; common types such as integers and strings do. You will learn more about these traits later.

## Practical example

Open [`examples/reading_tracker.rs`](examples/reading_tracker.rs):

```rust
use std::collections::{HashMap, HashSet};

fn main() {
    let mut reading_list = vec!["Rust Book", "Rust By Example"];
    reading_list.push("Standard Library Guide");

    let mut pages_read = HashMap::new();
    pages_read.insert("Rust Book", 35);
    pages_read.insert("Rust By Example", 18);
    pages_read.insert("Standard Library Guide", 4);

    let mut finished = HashSet::new();
    finished.insert("Rust By Example");

    for title in &reading_list {
        let pages = pages_read.get(title).copied().unwrap_or(0);
        let status = if finished.contains(title) {
            "finished"
        } else {
            "in progress"
        };
        println!("{title}: {pages} pages read ({status})");
    }
}
```

Compile and run from the repository root:

```sh
rustc days/11-collections/examples/reading_tracker.rs -o reading_tracker
./reading_tracker
```

Expected output:

```text
Rust Book: 35 pages read (in progress)
Rust By Example: 18 pages read (finished)
Standard Library Guide: 4 pages read (in progress)
```

The vector keeps titles in the order entered, so the output order is stable. The map provides each title's page count, while the set records which title is finished. `get` returns an `Option<&i32>`; `.copied()` turns that into an `Option<i32>` because integers are `Copy`, and `unwrap_or(0)` supplies a default if a title has no count.

## Common mistakes

- **Mixing element types in a vector:** A vector has one element type. Use an enum if entries can be one of several types, or a struct if each entry has several named fields.
- **Indexing without checking:** An invalid vector index panics. Use `.get(index)` and handle its `Option` when the index may be out of range.
- **Expecting `.get()` to return the value directly:** It returns `Option<&T>` (or `Option<&V>` for a map) because the item may be absent.
- **Forgetting a mutable vector binding:** Methods such as `push` and `pop` change the vector, so declare it with `let mut`.
- **Expecting a map or set to preserve order:** Hash collections do not guarantee iteration order. Use a vector when order matters.
- **Assuming inserting a duplicate map key creates another entry:** A key is unique; inserting it again replaces its value. A set also ignores duplicate values.
- **Holding an element reference while changing the vector:** A reference into a vector may prevent operations that could reallocate or change its storage. Finish using the reference before mutating the vector.

## Exercises

Try these before reading the [Day 11 solutions](../../solutions/11-collections.md).

1. Create a vector of five integers, add another value, and print the vector's length and final element.
2. Use `.get()` to look up an index beyond the end of a vector and handle both `Some` and `None` with `match`.
3. Create a hash map from three item names to quantities. Update one quantity and print it.
4. Count how many times each word appears in a short list of words using a `HashMap` and `entry`/`or_insert`.
5. Insert repeated words into a `HashSet` and print how many unique values remain.
6. Explain why a vector is a better choice than a hash map when the order of items matters.

## Further reading

- [The Rust Programming Language: Common Collections](https://doc.rust-lang.org/book/ch08-00-common-collections.html)
- [Standard library: `Vec`](https://doc.rust-lang.org/std/vec/struct.Vec.html)
- [Standard library: `HashMap`](https://doc.rust-lang.org/std/collections/struct.HashMap.html)
- [Standard library: `HashSet`](https://doc.rust-lang.org/std/collections/struct.HashSet.html)

## Course navigation

- [Course index](../../README.md)
- Previous: [Day 10: Enums and Pattern Matching](../10-enums-and-pattern-matching/README.md)
- Next: [Day 12: Error Handling](../12-error-handling/README.md)
