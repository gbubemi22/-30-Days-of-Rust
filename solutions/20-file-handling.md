# Day 20: File Handling — solutions

Try the [exercises](../days/20-file-handling/README.md#exercises) first. These examples use paths relative to the repository root.

## 1. Read the sample file

```rust
let text = std::fs::read_to_string("days/20-file-handling/examples/notes.txt")?;
println!("{text}");
```

Use this inside a function returning `std::io::Result<()>`.

## 2. Count lines and words

```rust
let lines = text.lines().count();
let words = text.split_whitespace().count();
println!("{lines} lines, {words} words");
```

## 3. Write and read in the temporary directory

```rust
let path = std::env::temp_dir().join("rust-course-practice.txt");
std::fs::write(&path, "saved text")?;
let saved = std::fs::read_to_string(path)?;
println!("{saved}");
```

## 4. Report a missing-file error

```rust
match std::fs::read_to_string("missing.txt") {
    Ok(text) => println!("{text}"),
    Err(error) => eprintln!("Could not read the file: {error}"),
}
```

## 5. Build and inspect a path

```rust
let path = std::path::Path::new("days").join("20-file-handling/examples/notes.txt");
println!("Exists: {}", path.exists());
```
