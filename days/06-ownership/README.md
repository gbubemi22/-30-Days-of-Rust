# Day 6: Ownership

Ownership is Rust's system for managing memory. It gives each value a clear owner and lets Rust know when that value can be cleaned up. Today you will learn the ownership rules, see how moving and copying differ, and return ownership from a function.

## Learning objectives

- describe stack and heap storage at a basic level;
- state Rust's three ownership rules;
- recognize when assigning or passing a value moves ownership;
- identify values that are copied instead of moved;
- return a value when a function needs to give ownership back.

## Prerequisites

Read [Day 5: Functions](../05-functions/README.md). You should know how to call a function and pass arguments.

## Why ownership?

Programs need memory to store values. The **stack** stores values with a known, fixed size and follows a last-in, first-out order. The **heap** stores data whose size can vary while the program runs; a `String`, for example, owns a growable text buffer stored on the heap.

Languages manage heap memory in different ways. Rust uses ownership rules checked by the compiler. This lets Rust know when a value is no longer needed and can be cleaned up, without a garbage collector that runs while the program executes.

## The ownership rules

Rust's core ownership rules are:

1. Each value has an owner.
2. There can only be one owner at a time.
3. When the owner goes out of scope, the value is dropped (cleaned up).

```rust
{
    let message = String::from("Hello");
    println!("{message}");
} // message goes out of scope here and its String is dropped
```

The braces create a scope. When execution leaves it, Rust knows that `message` is no longer usable and can release the `String`'s allocated memory.

## Moving ownership

For a heap-owning value such as `String`, assigning it to another variable moves ownership:

```rust
let first = String::from("hello");
let second = first;
println!("{second}");
```

After `let second = first`, `second` owns the string. `first` is no longer valid, so trying to use it would produce a compiler error. This prevents two owners from both trying to clean up the same allocation.

Passing an owned value to a function also moves it into the function:

```rust
fn display(text: String) {
    println!("{text}");
}

let note = String::from("Read chapter one");
display(note);
// note is no longer usable here
```

When the function ends, its `text` parameter goes out of scope and the string is dropped. Later, references will let us use a value without transferring ownership.

## Copying simple values

Some types, such as integers and booleans, have a known size and are inexpensive to copy. Assigning one copies the value, so both bindings remain usable:

```rust
let original = 8;
let copy = original;
println!("{original} and {copy}");
```

Types such as `i32` and `bool` implement the `Copy` behavior. `String` does not: copying its bits would create two owners of the same heap allocation. Cloning a `String` makes a separate allocation and copies its contents:

```rust
let original = String::from("hello");
let copy = original.clone();
```

Cloning can take time and memory, so use it when you really need an independent owned copy.

## Returning ownership

A function can return an owned value to its caller:

```rust
fn make_message() -> String {
    String::from("Welcome")
}

let message = make_message();
```

Ownership of the returned `String` moves to `message`. A function can also take ownership and return it, but this can become awkward when several values are involved. Borrowing, which we will learn next, avoids many such transfers.

## Practical example

Open [`examples/ownership.rs`](examples/ownership.rs):

```rust
fn add_exclamation(mut text: String) -> String {
    text.push('!');
    text
}

fn main() {
    let title = String::from("Learning Rust");
    let updated_title = add_exclamation(title);
    println!("{updated_title}");

    let chapters = 6;
    let completed_chapters = chapters;
    println!("Completed {completed_chapters} of {chapters} chapters");
}
```

Compile and run from the repository root:

```sh
rustc days/06-ownership/examples/ownership.rs -o ownership
./ownership
```

Expected output:

```text
Learning Rust!
Completed 6 of 6 chapters
```

`title` moves into the `text` parameter, which becomes the function's owner. The function changes the string and returns it; ownership then moves to `updated_title`. The integer `chapters` is copied into `completed_chapters`, so both names can still be used.

## Common mistakes

- **Using a moved value:** After moving a `String`, the old variable is invalid. Use the new owner, clone when a separate copy is truly needed, or borrow the value (Day 7).
- **Assuming every assignment copies:** Simple `Copy` types are copied; owned heap data such as `String` usually moves.
- **Cloning automatically to silence an error:** A clone allocates and copies data. First ask whether the value should be moved or borrowed instead.
- **Returning a value but then using its old name:** Returning transfers ownership to the caller. Use the returned binding from then on.
- **Confusing scope and ownership:** A value is dropped when its owner leaves scope, unless ownership has moved elsewhere.

## Exercises

Try these before reading the [Day 6 solutions](../../solutions/06-ownership.md).

1. Create a `String`, move it into a second variable, and print the new owner.
2. Copy an integer into a second variable, then print both values.
3. Write a function that takes ownership of a `String`, adds a period, and returns the updated string. Call it and print the returned value.
4. Uncomment the marked line below, compile, and read the error. Explain which binding owns the string after the move.

   ```rust
   let first = String::from("ownership");
   let second = first;
   // println!("{first}");
   println!("{second}");
   ```

5. In your own words, why does assigning a `String` move it while assigning an `i32` copies it?

## Further reading

- [The Rust Programming Language: What Is Ownership?](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)
- [Rust By Example: Ownership](https://doc.rust-lang.org/rust-by-example/scope/move.html)

## Course navigation

- [Course index](../../README.md)
- Previous: [Day 5: Functions](../05-functions/README.md)
- Next: [Day 7: References and Borrowing](../07-references-and-borrowing/README.md)
