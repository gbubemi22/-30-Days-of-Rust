# Day 4: Operators and Control Flow — solutions

Try the [exercises](../days/04-operators-and-control-flow/README.md#exercises) first. These are sample solutions; equivalent approaches are welcome.

## 1. Even or odd

```rust
let number = 14;
if number % 2 == 0 {
    println!("{number} is even");
} else {
    println!("{number} is odd");
}
```

An even integer has remainder zero when divided by two.

## 2. Choose the larger value

```rust
let first = 18;
let second = 23;
let larger = if first > second { first } else { second };
println!("Larger: {larger}");
```

Both branches return an integer, so the `if` expression has a consistent type.

## 3. Print 1 through 5

```rust
for number in 1..=5 {
    println!("{number}");
}
```

The `=` makes the upper bound inclusive.

## 4. Count up with `while`

```rust
let mut number = 1;
while number <= 3 {
    println!("{number}");
    number += 1;
}
```

Incrementing `number` eventually makes the condition false.

## 5. Sum until at least 10

```rust
let mut next = 1;
let mut total = 0;
let final_total = loop {
    total += next;
    if total >= 10 {
        break total;
    }
    next += 1;
};
println!("Total: {final_total}");
```

The loop adds `1`, `2`, `3`, and `4`, then `break total` exits the loop and supplies `10` as its value.

## 6. Choosing between `for` and `while`

Use `for` when visiting each item in a collection or a known range; it handles moving to the next item. Use `while` when repetition depends on a condition that is checked before each pass.
