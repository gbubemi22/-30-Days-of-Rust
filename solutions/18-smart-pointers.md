# Day 18: Smart Pointers — solutions

Try the [exercises](../days/18-smart-pointers/README.md#exercises) first. These are sample answers.

## 1. A boxed string

```rust
let message = Box::new(String::from("on the heap"));
println!("{message}");
```

## 2. Shared `Rc`

```rust
use std::rc::Rc;
let first = Rc::new(String::from("shared text"));
let second = Rc::clone(&first);
println!("{}", Rc::strong_count(&first)); // 2
```

## 3. Interior mutability

```rust
use std::cell::RefCell;
let value = RefCell::new(10);
*value.borrow_mut() += 5;
println!("{}", value.borrow());
```

## 4. Choosing `Rc` or `Arc`

Use `Rc` for shared ownership within one thread. Use `Arc` when owners may be in different threads; its reference count is updated atomically.

## 5. Runtime borrow checks

`RefCell` enforces the borrowing rules when `borrow` or `borrow_mut` is called. A conflicting borrow can only be detected then, so the program compiles but panics if the rule is broken at runtime.
