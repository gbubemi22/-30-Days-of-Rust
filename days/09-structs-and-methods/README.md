# Day 9: Structs and Methods

Tuples can group values, but their positions do not explain what those values mean. A **struct** lets you create a named type with named fields. Methods let you attach behavior to that type, so data and the operations that use it stay together.

## Learning objectives

- define and create a struct with named fields;
- use tuple structs and unit-like structs;
- read and update fields;
- define methods with `&self` and `&mut self`;
- define an associated function such as a constructor.

## Prerequisites

Read [Day 8: Strings and Slices](../08-strings-and-slices/README.md). You should understand variables, types, functions, and basic borrowing.

## Named-field structs

Define a struct with the `struct` keyword and name each field with a type:

```rust
struct Book {
    title: String,
    pages: u32,
    is_finished: bool,
}
```

Create a value by supplying each field:

```rust
let book = Book {
    title: String::from("Rust Basics"),
    pages: 240,
    is_finished: false,
};
println!("{} has {} pages", book.title, book.pages);
```

Each field has the type declared in the struct. If the instance binding is mutable, you can update its fields:

```rust
let mut book = Book {
    title: String::from("Rust Basics"),
    pages: 240,
    is_finished: false,
};
book.is_finished = true;
```

The whole instance must be mutable to change one of its fields. Fields are private or public depending on module visibility; modules and visibility are covered on Day 16.

## Tuple structs and unit-like structs

A tuple struct gives a tuple's fields a meaningful type name. Its fields are accessed by position:

```rust
struct Point(i32, i32);
let origin = Point(0, 0);
println!("{}, {}", origin.0, origin.1);
```

Even if two tuple structs contain the same field types, they are distinct types. A unit-like struct has no fields:

```rust
struct Marker;
let _marker = Marker;
```

Unit-like structs are useful when a type needs to represent an idea or behavior without storing data. For now, focus on named fields for ordinary records.

## Methods

An `impl` block defines methods associated with a type. A method's first parameter is usually `self`, which tells Rust which instance the method works with:

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
```

`&self` is shorthand for borrowing the instance immutably. The method can read its fields but not change them. Call it with dot syntax: `rectangle.area()`.

Use `&mut self` when a method must change the instance:

```rust
impl Rectangle {
    fn widen(&mut self, amount: u32) {
        self.width += amount;
    }
}
```

The instance must be mutable to call this method. A method can also take `self` by value, which consumes the instance; you will see that pattern in some APIs as you learn more Rust.

## Associated functions

An associated function belongs to the type but does not take `self`. It is often used to construct an instance:

```rust
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

let small = Rectangle::square(5);
```

Call an associated function with the type name and `::`, rather than dot syntax. Rust does not require constructors to be named `new`, but that is a common convention.

## Practical example

Open [`examples/reading_list.rs`](examples/reading_list.rs):

```rust
struct ReadingItem {
    title: String,
    pages: u32,
    pages_read: u32,
}

impl ReadingItem {
    fn new(title: &str, pages: u32) -> ReadingItem {
        ReadingItem {
            title: String::from(title),
            pages,
            pages_read: 0,
        }
    }

    fn progress(&self) -> u32 {
        self.pages_read * 100 / self.pages
    }

    fn read_pages(&mut self, count: u32) {
        self.pages_read = (self.pages_read + count).min(self.pages);
    }
}

fn main() {
    let mut book = ReadingItem::new("Rust in Small Steps", 100);
    book.read_pages(25);
    println!("{}: {}% complete", book.title, book.progress());
}
```

Compile and run from the repository root:

```sh
rustc days/09-structs-and-methods/examples/reading_list.rs -o reading_list
./reading_list
```

Expected output:

```text
Rust in Small Steps: 25% complete
```

`ReadingItem` groups the title and page counts. `new` is an associated function that builds a value. `progress` borrows the item immutably to calculate a result. `read_pages` borrows it mutably to update progress and caps the count at the total number of pages.

## Common mistakes

- **Leaving out a field when creating a struct:** A value must initialize every field unless you use struct update syntax (introduced later).
- **Trying to change a field through an immutable binding:** Use `let mut instance` when you need to update its fields.
- **Calling an associated function with dot syntax:** Use `TypeName::function(...)`; dot syntax is for methods called on an instance.
- **Using `self` when the method only needs to read:** `&self` borrows the instance without taking ownership. Use `&mut self` only when changing it.
- **Forgetting the instance parameter:** Methods that need an instance must declare `self`, `&self`, or `&mut self` as their first parameter.
- **Dividing by zero in a calculation:** The example assumes a positive page count. A production constructor should validate that input; error handling is covered on Day 12.

## Exercises

Try these before reading the [Day 9 solutions](../../solutions/09-structs-and-methods.md).

1. Define a `Movie` struct with a title, release year, and rating. Create one and print its fields.
2. Define a tuple struct `Rgb` with three `u8` fields and create a color value.
3. Add an `area` method to a `Rectangle` struct with `width` and `height` fields.
4. Add a mutable method that increases a rectangle's width by a supplied amount.
5. Add an associated function `square(size)` that returns a square `Rectangle`.
6. Extend `ReadingItem` with a method that reports whether all pages have been read.

## Further reading

- [The Rust Programming Language: Structs](https://doc.rust-lang.org/book/ch05-00-structs.html)
- [Rust By Example: Structures](https://doc.rust-lang.org/rust-by-example/custom_types/structs.html)

## Course navigation

- [Course index](../../README.md)
- Previous: [Day 8: Strings and Slices](../08-strings-and-slices/README.md)
- Next: [Day 10: Enums and Pattern Matching](../10-enums-and-pattern-matching/README.md)
