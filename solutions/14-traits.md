# Day 14: Traits — solutions

Try the [exercises](../days/14-traits/README.md#exercises) first. These are sample implementations.

## 1–3. Define, implement, and use `Describe`

```rust
trait Describe { fn description(&self) -> String; }
struct Book { title: String }
struct Song { title: String, artist: String }
impl Describe for Book {
    fn description(&self) -> String { format!("Book: {}", self.title) }
}
impl Describe for Song {
    fn description(&self) -> String { format!("{} by {}", self.title, self.artist) }
}
fn print_description<T: Describe>(item: &T) {
    println!("{}", item.description());
}
```

## 4. Derive common traits

```rust
#[derive(Debug, PartialEq)]
struct Point { x: i32, y: i32 }
let first = Point { x: 2, y: 3 };
let second = Point { x: 2, y: 3 };
println!("{first:?}");
assert_eq!(first, second);
```

## 5. Meaning of a bound

`T: Describe` guarantees that any type used as `T` has an implementation of `Describe`, so the function can call its `description` method.
