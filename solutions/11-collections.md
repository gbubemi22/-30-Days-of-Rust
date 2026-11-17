# Day 11: Collections — solutions

Try the [exercises](../days/11-collections/README.md#exercises) first. These are example solutions; output formatting can vary.

## 1. Grow a vector

```rust
let mut values = vec![2, 4, 6, 8, 10];
values.push(12);
println!("Length: {}", values.len());
println!("Last value: {}", values[values.len() - 1]);
```

The vector is non-empty, so its final valid index is its length minus one.

## 2. Safely look up an index

```rust
let values = vec![5, 10, 15];
match values.get(8) {
    Some(value) => println!("Found {value}"),
    None => println!("No value at that index"),
}
```

`get` returns `None` for an out-of-range index instead of panicking.

## 3. Store and update quantities

```rust
use std::collections::HashMap;

let mut stock = HashMap::new();
stock.insert("notebooks", 12);
stock.insert("pens", 30);
stock.insert("folders", 7);
stock.insert("pens", 28); // replaces the old quantity
println!("Pens: {}", stock.get("pens").unwrap());
```

## 4. Count words

```rust
use std::collections::HashMap;

let words = ["rust", "learn", "rust", "practice", "rust", "learn"];
let mut counts = HashMap::new();

for word in words.iter() {
    let count = counts.entry(*word).or_insert(0);
    *count += 1;
}

println!("rust appears {} times", counts.get("rust").unwrap());
```

Hash map iteration order is unspecified, so look up a particular key when you want predictable output.

## 5. Keep only unique words

```rust
use std::collections::HashSet;

let words = ["rust", "learn", "rust", "practice", "learn"];
let unique_words: HashSet<_> = words.iter().copied().collect();
println!("Unique words: {}", unique_words.len());
```

The set has three values. Alternatively, insert each word in a loop; duplicate insertions do not add another copy.

## 6. Keep an order with a vector

A vector preserves the order in which values are stored and is designed for visiting them in sequence. A hash map is organized by keys, and it does not promise a stable iteration order.
