# Day 25: Working with External Crates — solutions

Try the [exercises](../days/25-working-with-external-crates/README.md#exercises) first.

## 1. Dependency

The example declares `unicode-segmentation` in `[dependencies]`; its source imports the crate as `unicode_segmentation`.

## 2. Inspect dependencies

From the repository root, run:

```sh
cargo tree --manifest-path days/25-working-with-external-crates/examples/grapheme-count/Cargo.toml
```

Cargo prints the direct crate and the dependencies it brings in.

## 3. Find another method

The crate provides grapheme, word, and sentence boundary iterators. Read the current crate documentation and try a method with a small example.

## 4. Compare text counts

```rust
use unicode_segmentation::UnicodeSegmentation;
let text = "hi 👋";
println!("bytes: {}", text.len());
println!("scalar values: {}", text.chars().count());
println!("graphemes: {}", text.graphemes(true).count());
```

For this text the counts are 7 bytes, 4 scalar values, and 4 graphemes.

## 5. Lockfile purpose

The lockfile records exact dependency resolutions, including transitive dependencies, so application builds can use the same crate versions until dependencies are intentionally updated.
