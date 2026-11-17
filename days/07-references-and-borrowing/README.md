# Day 7: References and Borrowing

Ownership makes it clear who is responsible for a value. Often a function only needs to look at a value or make a temporary change, not take it away. A **reference** lets code use a value without owning it; using a reference is called **borrowing**.

## Learning objectives

- create and use immutable references;
- explain borrowing without transferring ownership;
- use mutable references to change a borrowed value;
- apply Rust's rule for mixing mutable and immutable references;
- understand why a reference cannot outlive the value it points to.

## Prerequisites

Read [Day 6: Ownership](../06-ownership/README.md). You should understand moves, `String`, and function parameters.

## Immutable references

Use `&` to create a reference. A function can take `&String` to read a string without taking ownership:

```rust
fn character_count(text: &String) -> usize {
    text.len()
}

let message = String::from("Hello");
let count = character_count(&message);
println!("{message} has {count} bytes");
```

The `&message` argument borrows `message`. The function receives a reference, not ownership of the string, so `message` can still be used afterward. Calling `len` returns the number of bytes in the string; for plain English text, that is often the same as the number of characters, but Unicode text can use multiple bytes per character.

In many function signatures, `&str` is more flexible than `&String` for read-only text. You can borrow a string slice from a `String`, and string literals are already `&str`. We'll cover strings and slices on Day 8.

## Mutable references

If a function needs to change a value, it can accept a mutable reference (`&mut T`). The original binding must be mutable, and the caller passes it with `&mut`:

```rust
fn add_suffix(text: &mut String) {
    text.push_str("!");
}

let mut message = String::from("Hello");
add_suffix(&mut message);
println!("{message}");
```

The function borrows the string mutably for the call. It can change the existing string, and the caller retains ownership.

## Borrowing rules

While a value is borrowed, Rust enforces these rules:

- You can have any number of immutable references (`&T`) at the same time.
- You can have one mutable reference (`&mut T`) at a time.
- You cannot use an immutable reference while a mutable reference to the same value is active.

These rules ensure code cannot make a change while another part of the program is relying on an unchanged view. The compiler determines how long each reference is used. In beginner examples, the borrow usually lasts until the reference's last use.

This is rejected because the immutable reference is used after the attempted mutable borrow:

```rust
let mut message = String::from("Hello");
let view = &message;
message.push('!'); // error: message is still immutably borrowed
println!("{view}");
```

Move the print before the mutation, or stop using `view` before mutating `message`. The compiler may point out where the borrow starts and where it is used again.

## References must not outlive their values

A reference must not be used after the value it points to has been dropped. Rust rejects code that would return a reference to a local variable that is about to leave scope:

```rust
// This does not compile:
// fn dangling_reference() -> &String {
//     let text = String::from("temporary");
//     &text
// }
```

The local `text` is dropped when the function ends, so returning a reference to it would leave the caller with an invalid reference. Return an owned `String` instead, or borrow a value whose lifetime is managed by the caller. Lifetime relationships are studied in Day 15.

## Practical example

Open [`examples/borrowing.rs`](examples/borrowing.rs):

```rust
fn word_count(text: &str) -> usize {
    text.split_whitespace().count()
}

fn add_period(text: &mut String) {
    text.push('.');
}

fn main() {
    let mut note = String::from("Rust makes ownership visible");
    let words = word_count(&note);
    add_period(&mut note);

    println!("{note}");
    println!("Word count: {words}");
}
```

Compile and run from the repository root:

```sh
rustc days/07-references-and-borrowing/examples/borrowing.rs -o borrowing
./borrowing
```

Expected output:

```text
Rust makes ownership visible.
Word count: 4
```

`word_count` borrows the text immutably and counts its whitespace-separated words. After that borrow is no longer used, `add_period` borrows the string mutably and adds punctuation. `note` remains the owner throughout, so it is still available to print afterward.

## Common mistakes

- **Passing an owned value where a reference is expected:** If the function signature takes `&String`, pass `&value`. The `&` borrows instead of moving.
- **Mutably borrowing an immutable binding:** Declare the owned value with `let mut` before passing `&mut value`.
- **Trying to create two active mutable references:** Keep one mutable borrow at a time; finish using it before borrowing mutably again.
- **Reading through an immutable reference while mutating:** Stop using the immutable reference before making the change.
- **Returning a reference to a local value:** The local is dropped at the end of its scope. Return an owned value or borrow data supplied by the caller.
- **Assuming `String::len()` counts visible characters:** It returns bytes. A Unicode character may take multiple bytes.

## Exercises

Try these before reading the [Day 7 solutions](../../solutions/07-references-and-borrowing.md).

1. Write a function that accepts `&String` and returns its byte length. Call it and then print the original string.
2. Write a function that accepts `&mut String` and appends a question mark. Call it and print the changed string.
3. Use a function that accepts `&str` with both a string literal and a `String`.
4. In a small program, create two immutable references to one string and use both.
5. Uncomment the mutation in the borrowing-rules example. Move code around so it compiles, then explain when the immutable borrow ends.

## Further reading

- [The Rust Programming Language: References and Borrowing](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)
- [Rust By Example: Borrowing](https://doc.rust-lang.org/rust-by-example/scope/borrow.html)

## Course navigation

- [Course index](../../README.md)
- Previous: [Day 6: Ownership](../06-ownership/README.md)
- Next: [Day 8: Strings and Slices](../08-strings-and-slices/README.md)
