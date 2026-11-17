# Day 4: Operators and Control Flow

Operators let you calculate and compare values. Control flow lets your program choose which code to run and how many times to repeat it. Today you will use arithmetic, comparisons, boolean logic, `if`, and Rust's three basic loop forms.

## Learning objectives

- use arithmetic, comparison, and logical operators;
- choose a branch with an `if` expression;
- repeat work with `loop`, `while`, and `for`;
- stop or skip an iteration with `break` and `continue`;
- recognize that Rust conditions must be boolean values.

## Prerequisites

Read [Day 3: Data Types](../03-data-types/README.md). You should be comfortable with numbers, booleans, and arrays.

## Operators

### Arithmetic

For integer and floating-point values, Rust supports addition (`+`), subtraction (`-`), multiplication (`*`), division (`/`), and remainder (`%`). Integer division drops the fractional part:

```rust
let whole = 7 / 2;       // 3
let remainder = 7 % 2;   // 1
let decimal = 7.0 / 2.0; // 3.5
```

Use parentheses to make the intended order clear. Division by zero is not a valid calculation; check cases where the divisor might be zero before dividing.

### Comparisons

Comparison operators produce a `bool`:

| Operator | Meaning |
| --- | --- |
| `==` | equal to |
| `!=` | not equal to |
| `<`, `<=` | less than, less than or equal to |
| `>`, `>=` | greater than, greater than or equal to |

Use `==` to compare values. A single `=` assigns a value to a variable; it does not test equality.

### Logical operators

Use `&&` for “and,” `||` for “or,” and `!` to negate a boolean:

```rust
let has_ticket = true;
let is_old_enough = true;
let can_enter = has_ticket && is_old_enough;
```

The right side of `&&` is evaluated only when needed: when the left side is false, the result is already false. Similarly, `||` skips its right side when the left side is true. This is called short-circuit evaluation.

## Choose a branch with `if`

An `if` runs one block when its condition is `true`, and an optional `else` block when it is `false`:

```rust
let temperature = 22;

if temperature > 25 {
    println!("It is warm.");
} else {
    println!("It is mild or cool.");
}
```

Rust conditions must be booleans. Unlike some languages, a number such as `1` is not treated as true. When an `if` is used to produce a value, each branch must produce the same type:

```rust
let message = if temperature > 25 { "warm" } else { "cool" };
```

## Repeat work with loops

### `loop`: repeat until stopped

`loop` repeats indefinitely until the code reaches `break`. A `break` can also return a value from the loop:

```rust
let result = loop {
    break 42;
};
```

### `while`: repeat while a condition holds

`while` checks a boolean condition before each pass:

```rust
let mut countdown = 3;
while countdown > 0 {
    println!("{countdown}");
    countdown -= 1;
}
```

The condition is checked again after every iteration. Make sure the loop can eventually make its condition false, or it will not stop.

### `for`: visit each item in a sequence

Use `for` to process every item in an array or range:

```rust
for number in 1..=3 {
    println!("{number}");
}
```

`1..=3` includes both endpoints and visits `1`, `2`, and `3`. The range `1..3` excludes its upper endpoint and visits `1` and `2`. A `for` loop is usually the clearest choice when you know which items you want to visit.

`continue` skips the rest of the current iteration and starts the next one. `break` exits the loop. Both work with these loop forms.

## Practical example

Open [`examples/scoreboard.rs`](examples/scoreboard.rs):

```rust
fn main() {
    let scores = [72, 45, 88, 60];
    let passing_score = 60;
    let mut passed = 0;

    for index in 0..scores.len() {
        let score = scores[index];
        let result = if score >= passing_score {
            "pass"
        } else {
            "retry"
        };
        println!("Score {score}: {result}");

        if score >= passing_score {
            passed += 1;
        }
    }

    let mut countdown = 3;
    while countdown > 0 {
        print!("{countdown} ");
        countdown -= 1;
    }
    println!("Go!");
    println!("Passing scores: {passed}");
}
```

Compile and run from the repository root:

```sh
rustc days/04-operators-and-control-flow/examples/scoreboard.rs -o scoreboard
./scoreboard
```

Expected output:

```text
Score 72: pass
Score 45: retry
Score 88: pass
Score 60: pass
3 2 1 Go!
Passing scores: 3
```

The `for` loop visits each valid array index, and `scores[index]` reads that score. The `if` expression chooses a text label, and the separate `if` increments the counter only for passing scores. The `while` loop prints a countdown and updates its condition value each time so it eventually ends.

## Common mistakes

- **Using `=` instead of `==`:** `=` assigns; `==` compares. A condition needs to evaluate to a boolean.
- **Using a number as an `if` condition:** Write a comparison such as `count > 0`, which produces a `bool`.
- **Returning different types from `if` branches:** If the `if` expression is used as a value, both branches must have the same type (or one branch may diverge, for example by returning).
- **Forgetting to update a `while` condition:** If the condition never becomes false, the loop continues forever.
- **Confusing inclusive and exclusive ranges:** `a..b` stops before `b`; `a..=b` includes `b`.
- **Assuming integer division keeps a fraction:** `7 / 2` is `3`; use floating-point operands when a fractional result is needed.

## Exercises

Try these before reading the [Day 4 solutions](../../solutions/04-operators-and-control-flow.md).

1. Given an integer, print whether it is even or odd using the remainder operator.
2. Use `if` to assign the larger of two integer values to a variable, then print it.
3. Use a `for` loop and an inclusive range to print the numbers from 1 through 5.
4. Use a `while` loop to count up from 1 to 3, printing each value.
5. Use `loop` to add numbers starting at 1. Stop when the total reaches at least 10, return that total from the loop, and print it.
6. In your own words, describe when a `for` loop is a better fit than a `while` loop.

## Further reading

- [The Rust Programming Language: Control Flow](https://doc.rust-lang.org/book/ch03-05-control-flow.html)
- [Rust By Example: Flow of Control](https://doc.rust-lang.org/rust-by-example/flow_control.html)

## Course navigation

- [Course index](../../README.md)
- Previous: [Day 3: Data Types](../03-data-types/README.md)
- Next: [Day 5: Functions](../05-functions/README.md)
