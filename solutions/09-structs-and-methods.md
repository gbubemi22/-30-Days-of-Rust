# Day 9: Structs and Methods — solutions

Try the [exercises](../days/09-structs-and-methods/README.md#exercises) first. These are sample implementations; field names and output wording can differ.

## 1. A `Movie` struct

```rust
struct Movie {
    title: String,
    release_year: u16,
    rating: f32,
}

fn main() {
    let movie = Movie {
        title: String::from("A Rusty Adventure"),
        release_year: 2026,
        rating: 8.5,
    };
    println!("{} ({}) — {}", movie.title, movie.release_year, movie.rating);
}
```

## 2. An RGB tuple struct

```rust
struct Rgb(u8, u8, u8);

let teal = Rgb(0, 128, 128);
println!("red={}, green={}, blue={}", teal.0, teal.1, teal.2);
```

Each channel fits in an unsigned 8-bit integer.

## 3. Rectangle area

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

## 4. Widen a rectangle

```rust
impl Rectangle {
    fn widen(&mut self, amount: u32) {
        self.width += amount;
    }
}
```

Call it on a mutable value, for example `let mut shape = Rectangle { width: 3, height: 4 }; shape.widen(2);`.

## 5. Construct a square

```rust
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

let shape = Rectangle::square(5);
```

`square` has no `self` parameter, so it is called through the type name.

## 6. Check whether reading is complete

```rust
impl ReadingItem {
    fn is_finished(&self) -> bool {
        self.pages_read == self.pages
    }
}
```

This method only reads fields, so it takes `&self`.
