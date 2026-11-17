# Day 15: Lifetimes — solutions

Try the [exercises](../days/15-lifetimes/README.md#exercises) first. These examples show relationships; lifetime names can differ.

## 1. Return the first input

```rust
fn first<'a>(left: &'a str, _right: &'a str) -> &'a str { left }
```

## 2. Return the longer input

```rust
fn longer<'a>(left: &'a str, right: &'a str) -> &'a str {
    if left.len() >= right.len() { left } else { right }
}
let result = longer("a short phrase", "Rust");
println!("{result}");
```

## 3. Store a borrowed quote

```rust
struct Quote<'a> { text: &'a str }
let source = String::from("Keep learning");
let quote = Quote { text: &source };
println!("{}", quote.text);
```

`source` stays alive while `quote` is used.

## 4. Why annotations cannot extend scope

A lifetime annotation is a promise about how references relate. It cannot prevent a local value from being dropped at the end of its scope; returning its reference would still be invalid.

## 5. Understand `choose`

The returned reference can point to `x`, because only `x` carries the output lifetime `'a`. The function cannot return `y` unless its lifetime is also related to `'a` in the signature.
