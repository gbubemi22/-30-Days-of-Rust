# Day 12: Error Handling — solutions

Try the [exercises](../days/12-error-handling/README.md#exercises) first. These are sample solutions; equivalent error handling is fine.

## 1. First element as an `Option`

```rust
fn first(values: &[i32]) -> Option<&i32> {
    values.first()
}
```

An empty slice has no first value, so the method returns `None`.

## 2. Parse an age

```rust
fn parse_age(text: &str) -> Result<u8, std::num::ParseIntError> {
    text.trim().parse::<u8>()
}
```

Values outside the `u8` range also produce a parse error.

## 3. Handle both outcomes

```rust
match parse_age("24") {
    Ok(age) => println!("Age: {age}"),
    Err(error) => println!("Invalid age: {error}"),
}
```

## 4. Parse and add with `?`

```rust
fn sum_text(first: &str, second: &str) -> Result<i32, std::num::ParseIntError> {
    let first = first.parse::<i32>()?;
    let second = second.parse::<i32>()?;
    Ok(first + second)
}
```

If either parse fails, `?` returns that error to the caller.

## 5. Choosing between `Option` and `Result`

Use `Option` when a value may simply be absent, such as searching a collection. Use `Result` when failure has useful error information or a caller may need to know why the operation failed.
