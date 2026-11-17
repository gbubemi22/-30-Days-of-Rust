# Day 17: Iterators and Closures — solutions

Try the [exercises](../days/17-iterators-and-closures/README.md#exercises) first. These are sample approaches.

## 1. Multiply by three

```rust
let triple = |number: i32| number * 3;
println!("{}", triple(7));
```

## 2. Square each number

```rust
let values = vec![2, 3, 4];
let squares: Vec<i32> = values.iter().map(|value| value * value).collect();
```

## 3. Keep longer words

```rust
let words = vec!["sun", "river", "mountain", "sky"];
let longer: Vec<&&str> = words.iter().filter(|word| word.len() > 4).collect();
```

The nested reference type is a consequence of filtering an iterator over references. You can also copy the items first if you prefer `Vec<&str>` as the output.

## 4. Parse all values

```rust
let inputs = ["4", "9", "12"];
let values: Result<Vec<i32>, _> = inputs.iter().map(|text| text.parse()).collect();
```

## 5. Why the source is still usable

`.iter()` creates an iterator of borrowed elements. The pipeline reads those values and does not take ownership of the original vector.
