# Day 10: Enums and Pattern Matching

An enum lets you describe a value that can be one of several named possibilities. Pattern matching lets your program respond to each possibility clearly. Today you will define enums, attach data to variants, and use `match` and `if let` to work with them.

## Learning objectives

- define an enum with one or more variants;
- create enum values and match on them;
- explain why `match` must cover every possible case;
- use enum variants that carry data;
- write a concise `if let` when only one case matters.

## Prerequisites

Read [Day 9: Structs and Methods](../09-structs-and-methods/README.md). You should know how to define a type and call functions.

## Define an enum

Use `enum` when a value should be exactly one of a set of alternatives:

```rust
enum Direction {
    North,
    South,
    East,
    West,
}

let heading = Direction::North;
```

Each alternative is called a **variant**. Write a variant using the enum name and `::`. Enums are useful when a value has a limited set of valid forms; they make those alternatives explicit in the type.

## Match every possibility

The `match` expression compares a value against patterns and runs the first matching arm:

```rust
let message = match heading {
    Direction::North => "Going north",
    Direction::South => "Going south",
    Direction::East => "Going east",
    Direction::West => "Going west",
};
```

Each arm has a pattern, `=>`, and an expression. Since `match` is an expression, its arms must produce compatible types when its value is used. Rust checks that every possible enum variant is handled. If a new variant is added later, the compiler helps find matches that need updating.

For values where a catch-all is appropriate, `_` matches anything not handled earlier:

```rust
match heading {
    Direction::North => println!("Up on the map"),
    _ => println!("Another direction"),
}
```

Use `_` only when the remaining cases truly share the same behavior; listing variants separately can make important cases clearer.

## Variants can hold data

Enum variants can each carry different kinds or amounts of data:

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(u8, u8, u8),
}
```

This enum represents messages that need no data, named fields, one value, or three values. A `match` can unpack the data:

```rust
let action = Message::Move { x: 3, y: 5 };
match action {
    Message::Move { x, y } => println!("Move to ({x}, {y})"),
    Message::Write(text) => println!("Text: {text}"),
    Message::ChangeColor(red, green, blue) => println!("RGB: {red}, {green}, {blue}"),
    Message::Quit => println!("Quit"),
}
```

Structs and enums can both hold data. Choose a struct when all fields describe one kind of value at the same time; choose an enum when a value is one of several alternatives.

## `if let` for one case

If you care about one variant and want to ignore the rest, `if let` is a shorter option:

```rust
if let Message::Write(text) = action {
    println!("Text: {text}");
}
```

You can add `else` for the other cases. Use `match` when you want the compiler to help ensure every case has been considered.

## The standard library's `Option`

Rust uses the enum `Option<T>` to represent a value that may or may not exist. It has two variants: `Some(value)` and `None`:

```rust
let maybe_score = Some(95);
let no_score: Option<i32> = None;

match maybe_score {
    Some(score) => println!("Score: {score}"),
    None => println!("No score yet"),
}
```

`Option` makes a missing value explicit instead of using a special null value. We will work more with `Option` and `Result` on Day 12.

## Practical example

Open [`examples/notifications.rs`](examples/notifications.rs):

```rust
enum Notification {
    FriendRequest(String),
    NewMessage { from: String, unread: u32 },
    Reminder,
}

fn describe(notification: Notification) -> String {
    match notification {
        Notification::FriendRequest(name) => format!("{name} sent a friend request"),
        Notification::NewMessage { from, unread } => {
            format!("{from} sent {unread} unread message(s)")
        }
        Notification::Reminder => String::from("You have a reminder"),
    }
}

fn main() {
    println!("{}", describe(Notification::FriendRequest(String::from("Ayo"))));
    println!(
        "{}",
        describe(Notification::NewMessage {
            from: String::from("Mina"),
            unread: 2,
        })
    );
    println!("{}", describe(Notification::Reminder));
}
```

Compile and run from the repository root:

```sh
rustc days/10-enums-and-pattern-matching/examples/notifications.rs -o notifications
./notifications
```

Expected output:

```text
Ayo sent a friend request
Mina sent 2 unread message(s)
You have a reminder
```

`Notification` has three variants, each shaped for a different kind of alert. `describe` uses `match` to handle all three and returns a `String`. Each call constructs one variant and passes ownership to the function.

## Common mistakes

- **Leaving out a variant from `match`:** Rust reports a non-exhaustive match. Add an arm for the missing variant or a deliberate catch-all.
- **Using the wrong enum name or variant:** Variants are qualified, for example `Notification::Reminder`.
- **Confusing a struct with an enum:** A struct has all its fields together; an enum value is one variant at a time.
- **Returning different types from match arms:** When the match produces a value, its arms need compatible result types.
- **Using `if let` when all cases need handling:** Prefer `match` when each alternative should have a deliberate outcome.
- **Forgetting to state the type of `None`:** Since `None` contains no value to infer from, Rust may need a type annotation such as `let item: Option<i32> = None;`.

## Exercises

Try these before reading the [Day 10 solutions](../../solutions/10-enums-and-pattern-matching.md).

1. Define a `TrafficLight` enum with red, yellow, and green variants. Match on each and print an action.
2. Define an enum `Shape` with a circle variant containing a radius and a rectangle variant containing width and height. Match on a value and print its dimensions.
3. Create an `Option<i32>` and use `match` to print the number or a message that it is missing.
4. Rewrite the previous exercise with `if let` to handle only `Some`.
5. Add a new variant to your `TrafficLight` enum and check that your `match` handles it.

## Further reading

- [The Rust Programming Language: Enums](https://doc.rust-lang.org/book/ch06-00-enums.html)
- [Rust By Example: Enums](https://doc.rust-lang.org/rust-by-example/custom_types/enum.html)
- [The Rust Programming Language: `match` Control Flow](https://doc.rust-lang.org/book/ch06-02-match.html)

## Course navigation

- [Course index](../../README.md)
- Previous: [Day 9: Structs and Methods](../09-structs-and-methods/README.md)
- Next: [Day 11: Collections](../11-collections/README.md)
