# Day 25: Working with External Crates

Rust's standard library covers many common needs, and the Rust ecosystem offers crates for additional capabilities. A good dependency choice solves a real problem, has clear documentation, and fits your project's maintenance needs. Today you will find a crate, add it with Cargo, understand version requirements, and inspect dependencies.

## Learning objectives

- distinguish the standard library from third-party crates;
- find a crate's purpose, documentation, license, and maintenance information;
- declare a dependency in `Cargo.toml`;
- understand a basic semantic version requirement;
- inspect and update dependencies with Cargo.

## Prerequisites

Read [Day 24: Macros](../24-macros/README.md). You should know the Cargo basics from Day 1 and have used a Cargo project such as the Serde example on Day 21.

## Finding a crate

The [crates.io registry](https://crates.io/) is Cargo's default public package registry. Before adding a crate, check:

- whether the standard library already meets the need;
- the crate's documentation and examples;
- its recent maintenance, supported Rust version, and license;
- its dependency tree, feature options, and security advisories where relevant.

Crate downloads and popularity are useful signals, but they do not replace reviewing whether the crate fits your project.

## Add a dependency

Dependencies are declared in `Cargo.toml`. This lesson uses `unicode-segmentation`, which can count user-perceived text graphemes rather than UTF-8 bytes or Unicode scalar values:

```toml
[dependencies]
unicode-segmentation = "1"
```

Cargo version requirements follow semantic versioning conventions. The requirement `"1"` allows compatible 1.x releases; `Cargo.lock` records the precise selected version for an application build. A library generally commits its manifest but often does not commit a lockfile, depending on project policy.

## Use the crate

The example project is in [`examples/grapheme-count/`](examples/grapheme-count/). It imports the `UnicodeSegmentation` trait to count grapheme clusters:

```rust
use unicode_segmentation::UnicodeSegmentation;

fn main() {
    let text = "hi 👋";
    println!("bytes: {}", text.len());
    println!("graphemes: {}", text.graphemes(true).count());
}
```

Run it from the repository root:

```sh
cargo run --manifest-path days/25-working-with-external-crates/examples/grapheme-count/Cargo.toml
```

Expected output:

```text
bytes: 7
graphemes: 4
```

The manifest declares the package and its dependency. Cargo downloads and builds the crate, and the source imports its library name using underscores even though the package name contains a hyphen.

## Dependency hygiene

- Add a dependency only when it provides meaningful value beyond the standard library.
- Prefer explicit feature selection when a crate offers optional functionality.
- Read release notes before major upgrades and review changes in `Cargo.lock`.
- Use `cargo tree` to inspect direct and transitive dependencies; use `cargo tree -d` to find duplicate versions.
- Keep secrets out of manifests and source files. Review licenses and update dependencies as part of project maintenance.

## Common mistakes

- **Writing the crate name incorrectly in source:** Cargo package names with hyphens are commonly imported with underscores.
- **Adding a dependency but not fetching/building it:** Run Cargo to resolve and compile dependencies.
- **Treating a version string as an exact pin:** Cargo requirements commonly allow compatible updates; the lockfile records the resolved version for applications.
- **Adding a crate for a task the standard library already handles well:** Extra dependencies increase build and maintenance work.
- **Copying a code sample without checking its version/features:** Ensure the sample matches the declared crate version and feature set.

## Exercises

Try these before reading the [Day 25 solutions](../../solutions/25-working-with-external-crates.md).

1. Inspect the example's `Cargo.toml` and identify its external dependency.
2. Run `cargo tree` with the example manifest and inspect the dependency graph.
3. Use the crate documentation to find another method on `UnicodeSegmentation`.
4. Change the sample text and compare byte length, scalar value count, and grapheme count.
5. Explain what `Cargo.lock` contributes to a binary application.

## Further reading

- [Cargo Book: Dependencies](https://doc.rust-lang.org/cargo/guide/dependencies.html)
- [Cargo Book: Specifying Dependencies](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html)
- [`unicode-segmentation` documentation](https://docs.rs/unicode-segmentation/)

## Course navigation

- [Course index](../../README.md)
- Previous: [Day 24: Macros](../24-macros/README.md)
- Next: [Day 26: Environment and Configuration](../26-environment-and-configuration/README.md)
