# Day 19: Testing and Debugging — solutions

Try the [exercises](../days/19-testing-and-debugging/README.md#exercises) first.

## 1. Test a negative input

```rust
#[test]
fn adds_negative_values() {
    assert_eq!(add(-2, 5), 3);
}
```

## 2. Test the larger value

```rust
pub fn larger(left: i32, right: i32) -> i32 {
    if left > right { left } else { right }
}

#[test]
fn chooses_larger_in_either_order() {
    assert_eq!(larger(8, 3), 8);
    assert_eq!(larger(3, 8), 8);
}
```

## 3. Check a boundary

```rust
#[test]
fn zero_plus_zero_is_zero() {
    assert_eq!(add(0, 0), 0);
}
```

## 4–5. Inspect the test feedback

When the expected value is wrong, Cargo reports the assertion location and shows left (actual) and right (expected). Restore the intended expectation. `dbg!(value)` prints the value and source location while returning the value for further use; remove it after inspection.
