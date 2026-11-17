# Day 20: File Handling

Files let a program keep information after it exits and read data prepared outside the program. Rust's standard library provides file and path types, along with `Result` values for reporting I/O failures. Today you will read and write a text file, work with paths, and handle errors.

## Learning objectives

- read a UTF-8 text file with `std::fs`;
- write text to a file;
- build paths with `Path` and `PathBuf`;
- propagate I/O errors using `Result` and `?`;
- recognize that a directory listing can fail too.

## Prerequisites

Read [Day 19: Testing and Debugging](../19-testing-and-debugging/README.md) and [Day 12: Error Handling](../12-error-handling/README.md). You should understand `Result` and the `?` operator.

## Paths

A path identifies a file or directory. Use `Path` to inspect a path and `PathBuf` to build or own one. Avoid joining path parts by manually adding `/`; path separators differ between operating systems:

```rust
use std::path::PathBuf;

let mut path = PathBuf::from("data");
path.push("notes.txt");
```

Relative paths are resolved from the program's current working directory, not automatically from the source file's location. This matters when you run a program from a different directory.

## Read and write text

`std::fs::read_to_string` reads a UTF-8 file into a `String`. `std::fs::write` creates or replaces a file with the provided contents. Both return `Result` because file operations can fail:

```rust
use std::fs;
use std::io;

fn copy_text(source: &str, destination: &str) -> io::Result<()> {
    let contents = fs::read_to_string(source)?;
    fs::write(destination, contents)?;
    Ok(())
}
```

`?` returns an error to the caller. The `io::Result<()>` return type means the function either completes successfully without a value or returns an I/O error.

For large files, reading the entire file into memory may be unsuitable. The standard library also provides buffered readers and writers for processing data incrementally.

## Directories and metadata

The `std::fs` module can create directories, inspect metadata, and list directory entries. These operations can fail because the path may be missing or permissions may be insufficient, so handle their `Result` values too.

## Practical example

Open [`examples/file_summary.rs`](examples/file_summary.rs). It reads a lesson sample, writes a short summary into the system temporary directory, then reads that summary back:

```rust
use std::fs;
use std::io;
use std::path::PathBuf;

fn main() -> io::Result<()> {
    let source = "days/20-file-handling/examples/notes.txt";
    let contents = fs::read_to_string(source)?;
    let summary = format!("{} lines read", contents.lines().count());

    let mut output = PathBuf::from(std::env::temp_dir());
    output.push("rust-course-file-summary.txt");
    fs::write(&output, summary)?;

    let saved = fs::read_to_string(&output)?;
    println!("{saved}");
    Ok(())
}
```

Run from the repository root:

```sh
rustc days/20-file-handling/examples/file_summary.rs -o file_summary
./file_summary
```

Expected output:

```text
3 lines read
```

The source path is relative to the repository root, so run the program from there. The output path is built in the system's temporary directory. Each file operation uses `?`, so an error is returned from `main` and reported by the runtime instead of being ignored.

## Common mistakes

- **Running with an unexpected working directory:** Relative paths start from the current directory. Run from the documented location or construct an absolute path.
- **Assuming a file always exists:** Reading may fail. Return or handle the `io::Error`.
- **Ignoring write behavior:** `fs::write` replaces an existing file's contents. Use `OpenOptions` when you need append or more control.
- **Joining a path with string separators:** Use `PathBuf::push` or `Path::join` to respect the platform.
- **Reading a large file all at once:** Use buffered reading when streaming is more appropriate.

## Exercises

Try these before reading the [Day 20 solutions](../../solutions/20-file-handling.md).

1. Read `notes.txt` and print its contents.
2. Count the number of lines and words in the file.
3. Write a new file in the temporary directory and read it back.
4. Try to read a file that does not exist and print a helpful error instead of panicking.
5. Use `PathBuf` to build a path from a directory and filename, then print whether it exists.

## Further reading

- [The Rust Programming Language: I/O Project](https://doc.rust-lang.org/book/ch12-02-reading-a-file.html)
- [Standard library: `std::fs`](https://doc.rust-lang.org/std/fs/index.html)
- [Standard library: `std::path`](https://doc.rust-lang.org/std/path/index.html)

## Course navigation

- [Course index](../../README.md)
- Previous: [Day 19: Testing and Debugging](../19-testing-and-debugging/README.md)
- Next: [Day 21: Serialization and Deserialization](../21-serialization-and-deserialization/README.md)
