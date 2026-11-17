# Day 16: Modules and Packages

As a project grows, putting everything in one file becomes hard to navigate. Rust modules group related code, control visibility, and provide paths to items. Cargo packages and crates organize projects and their dependencies. Today you will use `mod`, `pub`, paths, and `use`, and learn how these pieces fit together.

## Learning objectives

- distinguish a package, crate, and module;
- declare modules and use paths to reach items;
- control visibility with `pub`;
- bring a path into scope with `use`;
- recognize Cargo's role in organizing a package.

## Prerequisites

Read [Day 15: Lifetimes](../15-lifetimes/README.md). You should be comfortable with functions, structs, and enums.

## Crates, packages, and modules

A **crate** is a compilation unit: a binary crate builds an executable, and a library crate provides code for other crates. A **package** is a Cargo unit described by `Cargo.toml`; it can contain one library crate and multiple binary crates. A **module** organizes items within a crate.

Cargo creates conventional files such as `src/main.rs` for a binary or `src/lib.rs` for a library. In a source file, `mod catalog;` declares a module whose contents can live in `catalog.rs` or `catalog/mod.rs` beside the declaring file.

## Paths and visibility

An item inside a module is reached through its path. `crate::` starts at the current crate's root. Items are private by default to their module and descendants; mark an item `pub` to make it accessible from outside its defining module:

```rust
mod inventory {
    pub struct Item { pub name: String }
    pub fn item_count() -> usize { 3 }
}

fn main() {
    let item = inventory::Item { name: String::from("Notebook") };
    println!("{}: {}", item.name, inventory::item_count());
}
```

The module, struct, its field, and function must all be visible where used. Making a type public does not automatically make its fields public.

## `use` and nested modules

`use` brings a path into the current scope so you can refer to an item by a shorter name:

```rust
use std::collections::HashMap;
```

You can also nest modules to reflect related areas of a program. Paths use `::` between modules and items. Prefer explicit paths when they make the origin of a name clearer.

## Practical example

The example uses [`examples/modules.rs`](examples/modules.rs) and [`examples/catalog.rs`](examples/catalog.rs). Compile it from the repository root:

```sh
rustc days/16-modules-and-packages/examples/modules.rs -o modules
./modules
```

Expected output:

```text
Available items: 2
First item: Notebook
```

`modules.rs` declares `mod catalog;`, so Rust loads the sibling `catalog.rs`. The module keeps catalog data and behavior together. `pub` exposes the `Item` type, its `name` field, and the `items` function; `use` shortens the path to `Item`.

## Common mistakes

- **Declaring a module but placing its file elsewhere:** By default, the source file must follow Rust's module file layout.
- **Trying to use a private item from its parent or another module:** Add `pub` to the item that needs external visibility, not only to its enclosing type.
- **Confusing `use` with module declaration:** `mod` declares a module; `use` imports a path into scope.
- **Treating a package and crate as synonyms:** A package is Cargo's project unit; it contains one or more crates.
- **Making everything public:** Keep internal implementation details private unless callers need them.

## Exercises

Try these before reading the [Day 16 solutions](../../solutions/16-modules-and-packages.md).

1. Create a module `greetings` with a public function and call it from `main`.
2. Add a public struct with a private field. What can code outside the module read or construct?
3. Use `use` to bring a standard-library type into scope and refer to it without its full path.
4. Create a two-file module with `main.rs` and a sibling module file.
5. Explain the difference between a package, crate, and module.

## Further reading

- [The Rust Programming Language: Packages, Crates, and Modules](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html)
- [The Cargo Book: Packages and Crates](https://doc.rust-lang.org/cargo/guide/creating-a-new-project.html)

## Course navigation

- [Course index](../../README.md)
- Previous: [Day 15: Lifetimes](../15-lifetimes/README.md)
- Next: [Day 17: Iterators and Closures](../17-iterators-and-closures/README.md)
