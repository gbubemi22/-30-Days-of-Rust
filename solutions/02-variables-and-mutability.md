# Day 2: Variables and Mutability — solutions

Try the [exercises](../days/02-variables-and-mutability/README.md#exercises) first. These are sample solutions; your names and formatting can differ.

## 1. An immutable city

```rust
let city = "Lagos";
println!("{city}");
```

There is no `mut` because the binding is not reassigned.

## 2. A changing visit count

```rust
let mut visits = 0;
visits += 1;
println!("Visits: {visits}");
```

The binding is mutable because `visits` is reassigned by `+=`.

## 3. A constant

```rust
const DAYS_PER_WEEK: u32 = 7;
println!("Days per week: {DAYS_PER_WEEK}");
```

The type is explicit, as required for constants.

## 4. Shadow text with a number

```rust
let count = "41";
let count: u32 = count.parse().expect("count should contain a number");
println!("Next count: {}", count + 1);
```

The second `let` shadows the text binding with a new `u32` binding. `parse` can fail, so `expect` makes this small example stop with a message if the text is not a number. Error handling is covered on Day 12.

## 5. `mut` and shadowing

`mut` lets a binding be reassigned. Shadowing declares a new binding with the same name; it may have a different value or type, while the old binding is no longer the one referred to by that name.
