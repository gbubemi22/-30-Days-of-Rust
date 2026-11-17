# Day 18: Smart Pointers

A pointer is a value that refers to data elsewhere. Rust's **smart pointers** also manage ownership rules or provide extra behavior. Today you will meet `Box<T>`, `Rc<T>`, `Arc<T>`, and `RefCell<T>`, and learn when each is useful.

## Learning objectives

- explain why a `Box<T>` stores a value on the heap;
- use `Rc<T>` for shared ownership in one thread;
- use `RefCell<T>` for checked-at-runtime interior mutability;
- recognize `Arc<T>` as an atomically reference-counted pointer for sharing across threads;
- choose a pointer type based on the ownership problem.

## Prerequisites

Read [Day 17: Iterators and Closures](../17-iterators-and-closures/README.md). You should understand ownership, borrowing, and structs.

## `Box<T>`: owned heap data

`Box<T>` owns a value stored on the heap. Use it when you need a fixed-size pointer to heap data, such as a large value, a recursive type, or a value whose size is not known at compile time:

```rust
let number = Box::new(5);
println!("{number}");
```

When the box goes out of scope, it drops its value and frees the allocation. Rust's dereference coercions let you use many boxed values like the value inside them.

## `Rc<T>`: shared ownership in one thread

`Rc<T>` (reference counted) lets multiple owners share a value in a single-threaded program. The allocation is dropped after the last `Rc` owner goes away:

```rust
use std::rc::Rc;

let first = Rc::new(String::from("shared"));
let second = Rc::clone(&first);
println!("owners: {}", Rc::strong_count(&first));
```

`Rc::clone` increments the reference count; it does not clone the inner `String`. `Rc<T>` provides shared immutable access. It is not safe for sharing across threads.

## `RefCell<T>`: interior mutability

Sometimes a value is shared through an immutable reference but still needs to be changed. `RefCell<T>` allows this in a single-threaded program by checking borrowing rules at runtime instead of compile time:

```rust
use std::cell::RefCell;

let value = RefCell::new(5);
*value.borrow_mut() += 1;
println!("{}", value.borrow());
```

If code violates the borrowing rules, `RefCell` panics at runtime. Use it when you need interior mutability and can keep those runtime checks clear and controlled.

`Rc<RefCell<T>>` combines shared ownership with interior mutability in one thread. This pattern is useful in some graph or UI data structures, but can make ownership harder to follow; use ordinary references when they are sufficient.

## `Arc<T>`: shared ownership across threads

`Arc<T>` is an atomically reference-counted pointer. Its count can be updated safely across threads, so it is used when multiple threads need shared ownership. `Arc<T>` alone does not make the inner value mutable; shared mutation commonly uses synchronization types such as `Mutex<T>`, covered on Day 22.

## Practical example

Open [`examples/shared_notes.rs`](examples/shared_notes.rs):

```rust
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let notes = Rc::new(RefCell::new(vec![String::from("learn ownership")]));
    let first_view = Rc::clone(&notes);
    let second_view = Rc::clone(&notes);

    first_view.borrow_mut().push(String::from("practice borrowing"));
    println!("Notes: {:?}", second_view.borrow());
    println!("Owners: {}", Rc::strong_count(&notes));
}
```

Compile and run from the repository root:

```sh
rustc days/18-smart-pointers/examples/shared_notes.rs -o shared_notes
./shared_notes
```

Expected output:

```text
Notes: ["learn ownership", "practice borrowing"]
Owners: 3
```

The `Rc` values share ownership of one `RefCell<Vec<String>>`. `borrow_mut` checks at runtime that no conflicting borrow is active, then allows the vector to be changed. All three owners see the same updated notes.

## Common mistakes

- **Using `Rc<T>` across threads:** `Rc` is single-threaded. Use `Arc<T>` when shared ownership crosses thread boundaries.
- **Expecting `Arc<T>` to make mutation safe by itself:** Use a synchronization type such as `Mutex<T>` around shared mutable data.
- **Assuming `RefCell<T>` checks borrows at compile time:** Its checks happen at runtime, and a violation panics.
- **Using smart pointers without needing their behavior:** Prefer ordinary values and references until heap indirection, shared ownership, or interior mutability solves a real need.
- **Confusing `Rc::clone` with cloning the inner value:** It creates another shared owner and increments the reference count.

## Exercises

Try these before reading the [Day 18 solutions](../../solutions/18-smart-pointers.md).

1. Store a `String` in a `Box` and print the value.
2. Create two `Rc` owners for one string and print the strong reference count.
3. Use `RefCell<i32>` to update a value through `borrow_mut` and then read it.
4. In your own words, explain when to choose `Rc` versus `Arc`.
5. Explain why `RefCell` can panic even though its borrow calls compile.

## Further reading

- [The Rust Programming Language: Smart Pointers](https://doc.rust-lang.org/book/ch15-00-smart-pointers.html)
- [Rust By Example: Box](https://doc.rust-lang.org/rust-by-example/std/box.html)
- [Standard library: `Rc`](https://doc.rust-lang.org/std/rc/struct.Rc.html), [`RefCell`](https://doc.rust-lang.org/std/cell/struct.RefCell.html), and [`Arc`](https://doc.rust-lang.org/std/sync/struct.Arc.html)

## Course navigation

- [Course index](../../README.md)
- Previous: [Day 17: Iterators and Closures](../17-iterators-and-closures/README.md)
- Next: [Day 19: Testing and Debugging](../19-testing-and-debugging/README.md)
