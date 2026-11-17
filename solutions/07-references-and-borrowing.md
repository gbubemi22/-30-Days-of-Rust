# Day 7: References and Borrowing — solutions

Try the [exercises](../days/07-references-and-borrowing/README.md#exercises) first. These are sample solutions; equivalent code is fine.

## 1. Borrow a string to count bytes

```rust
fn byte_length(text: &String) -> usize {
    text.len()
}

fn main() {
    let message = String::from("Borrow me");
    println!("Bytes: {}", byte_length(&message));
    println!("Original: {message}");
}
```

The function borrows the string, so the caller remains its owner.

## 2. Mutably borrow to append punctuation

```rust
fn ask(text: &mut String) {
    text.push('?');
}

fn main() {
    let mut message = String::from("Ready");
    ask(&mut message);
    println!("{message}");
}
```

## 3. Accept a string slice

The example's signature already accepts `&str`:

```rust
fn word_count(text: &str) -> usize {
    text.split_whitespace().count()
}

fn main() {
    let owned = String::from("two words");
    println!("{}", word_count("three short words"));
    println!("{}", word_count(&owned));
}
```

Both a string literal and a borrow of a `String` can be used as `&str`.

## 4. Two immutable borrows

```rust
let message = String::from("shared reading");
let first_view = &message;
let second_view = &message;
println!("{first_view} / {second_view}");
```

Multiple immutable references can coexist.

## 5. End the immutable borrow before changing the string

```rust
let mut message = String::from("Hello");
let view = &message;
println!("{view}");
message.push('!');
println!("{message}");
```

The last use of `view` is the first `println!`. After that use, the borrow is no longer needed, so the mutation is allowed.
