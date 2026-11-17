# Day 5: Functions

Functions give a name to a set of instructions so you can reuse them and break a program into understandable pieces. Today you will define functions, pass information through parameters, return values, and distinguish expressions from statements.

## Learning objectives

- define and call a function;
- declare parameters and their types;
- give a function a return type;
- distinguish a returned expression from a statement ending in a semicolon;
- understand how variables inside a function have their own scope.

## Prerequisites

Read [Day 4: Operators and Control Flow](../04-operators-and-control-flow/README.md). You should know variables, types, and basic `if` expressions.

## Defining and calling functions

Use `fn` to define a function. `main` is the function where an executable program begins:

```rust
fn main() {
    say_hello();
}

fn say_hello() {
    println!("Hello!");
}
```

Rust allows you to define `say_hello` after `main`; functions do not need to appear in the order they are called. The parentheses in `say_hello()` call the function.

## Parameters

Parameters are named inputs to a function. Each parameter needs a type:

```rust
fn greet(name: &str) {
    println!("Hello, {name}!");
}

fn main() {
    greet("Mina");
}
```

The value passed when calling the function is called an **argument**. Here, `name` is the parameter and `"Mina"` is the argument. If a function takes multiple parameters, separate them with commas and declare a type for each.

## Return values

A function can send a value back to its caller. Write an arrow and the return type in the function signature:

```rust
fn double(number: i32) -> i32 {
    number * 2
}
```

The final expression in the function body, without a semicolon, is returned automatically. You can also return early with `return value;`, but the final-expression style is common for simple functions.

```rust
let answer = double(6);
```

The `-> i32` says that `double` returns an `i32`. If the function does not return a useful value, its return type is `()` (pronounced “unit”), and you can usually omit it from the signature.

## Expressions and statements

An **expression** evaluates to a value. A **statement** performs an action and does not produce a value for its surrounding code. A semicolon turns an expression into a statement:

```rust
let total = 3 + 4; // the let declaration is a statement
total + 1          // expression: its value is 8
```

That distinction matters at the end of a function. This returns the sum:

```rust
fn add(first: i32, second: i32) -> i32 {
    first + second
}
```

Adding a semicolon after `first + second` makes the body evaluate to `()` instead of `i32`, causing a return-type mismatch. The compiler often reports that it expected an integer but found `()`.

## Scope

The **scope** of a name is the part of the program where that name can be used. A parameter and variables declared inside a function are local to that function:

```rust
fn show_message() {
    let message = "Ready";
    println!("{message}");
}
```

`message` is available inside `show_message`, but not in `main` or another function. Pass values as parameters and return results instead of trying to use another function's local variables.

## Practical example

Open [`examples/receipt.rs`](examples/receipt.rs):

```rust
fn line_total(price: f64, quantity: u32) -> f64 {
    price * quantity as f64
}

fn print_receipt(item: &str, price: f64, quantity: u32) {
    let total = line_total(price, quantity);
    println!("{item}: {quantity} × {price:.2} = {total:.2}");
}

fn main() {
    print_receipt("Notebook", 2.50, 3);
    print_receipt("Pen", 1.25, 2);
}
```

Compile and run from the repository root:

```sh
rustc days/05-functions/examples/receipt.rs -o receipt
./receipt
```

Expected output:

```text
Notebook: 3 × 2.50 = 7.50
Pen: 2 × 1.25 = 2.50
```

`line_total` receives a price and quantity, then returns their product. The quantity is converted to `f64` so it can be multiplied by the floating-point price; Rust does not automatically mix integer and floating-point types. `print_receipt` calls `line_total` and displays the result. It has no explicit return type because it only performs an action. `main` calls the same function twice with different arguments.

## Common mistakes

- **Forgetting parameter types:** Every function parameter needs a type in its definition.
- **Calling with the wrong number or type of arguments:** Compare the call with the function signature; Rust reports the expected and received arguments.
- **Adding a semicolon to the returned expression:** A trailing semicolon discards the value, so a function declared to return a value instead produces `()`.
- **Returning the wrong type:** The value from the final expression must match the type after `->`.
- **Using a local variable outside its scope:** A function's local names are not visible elsewhere. Pass needed data as arguments or return it.
- **Mixing numeric types:** Convert deliberately, as with `quantity as f64`. Casting is introduced here only for this simple example; later lessons discuss conversions in more depth.

## Exercises

Try these before reading the [Day 5 solutions](../../solutions/05-functions.md).

1. Write a function named `print_welcome` that prints a welcome message, then call it from `main`.
2. Write `square` that accepts an `i32` and returns its square. Call it and print the result.
3. Write `is_even` that accepts an `i32` and returns a `bool`.
4. Write `describe_book` that accepts a title (`&str`) and page count (`u32`) and prints a sentence about the book.
5. Write a function that accepts two `f64` values and returns their average. Call it with `8.0` and `10.0`.
6. Remove the semicolon from, then add it to, the final expression of one of your value-returning functions. Read the compiler message and explain what changed.

## Further reading

- [The Rust Programming Language: Functions](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html)
- [Rust By Example: Functions](https://doc.rust-lang.org/rust-by-example/fn.html)

## Course navigation

- [Course index](../../README.md)
- Previous: [Day 4: Operators and Control Flow](../04-operators-and-control-flow/README.md)
- Next: [Day 6: Ownership](../06-ownership/README.md)
