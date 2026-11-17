# Day 1: Introduction to Rust

Welcome to the first day of **30 Days of Rust**. Today you will learn what Rust is, where it is useful, how its main tools fit together, and how to run your first Rust program.

## Learning objectives

By the end of this lesson, you should be able to:

- describe Rust in plain language and name a few kinds of programs it can build;
- explain the roles of `rustup`, `rustc`, and Cargo;
- compile and run a small Rust program;
- find the Rust compiler's version and ask it for help.

## Prerequisites

No Rust experience is needed. You will need a terminal (such as Terminal, PowerShell, or a Linux shell) and permission to install software on your computer.

## What is Rust?

Rust is a programming language for building software that runs efficiently and handles memory carefully. It is used for command-line tools, desktop and server software, embedded devices, game tools, and WebAssembly programs.

Rust checks many mistakes while compiling your program. Its ownership and type systems help prevent common memory and concurrency bugs without requiring a garbage collector. These checks can take time to learn; compiler messages are part of the learning process, and you can work through them one at a time.

Rust is a language, not a web framework or a particular kind of application. You can use it for many kinds of software.

## Install the Rust tools

The recommended installer is **rustup**. It installs and manages Rust toolchains, including the compiler and Cargo. Follow the official [Rust installation guide](https://www.rust-lang.org/tools/install) for your operating system. On Windows, the guide explains the required build tools as well.

After installation, open a new terminal and check the tools:

```text
rustc --version
cargo --version
rustup --version
```

Each command should print a version. If the terminal says a command cannot be found, follow the installer's PATH instructions and reopen the terminal.

### The three tools

- **rustup** manages Rust versions and related components. For example, `rustup update` updates the installed stable toolchain.
- **rustc** is the Rust compiler. It turns Rust source code into a program.
- **Cargo** is Rust's build tool and package manager. It creates projects, builds them, runs programs, and manages dependencies.

For a one-file introduction, we will call `rustc` directly. Most real projects use Cargo, which we will introduce in the next step of learning how Rust projects are organized.

## Your first program

Open [`examples/hello_world.rs`](examples/hello_world.rs). It contains:

```rust
fn main() {
    println!("Hello, Rust!");
}
```

`fn main()` defines the program's starting point. When the program runs, Rust begins by executing the statements inside `main`.

`println!` displays text followed by a newline. The `!` tells you that `println!` is a **macro**: Rust syntax that generates code. You do not need to understand macros yet; for now, use `println!` to display text.

### Compile and run it

From the repository's root directory, run:

```sh
rustc days/01-introduction/examples/hello_world.rs -o hello_world
./hello_world
```

On Windows Command Prompt, run the compiled program with `hello_world.exe`. The expected output is:

```text
Hello, Rust!
```

The first command compiles the source file and writes an executable named `hello_world` (or `hello_world.exe` on Windows). The second command runs it. If you omit `-o hello_world`, the compiler chooses a default executable name based on the source file.

## Common mistakes

- **`rustc` is not recognized / command not found:** Rust may not be installed, or the terminal may not have refreshed its PATH. Reopen the terminal and revisit the official installation instructions.
- **The source file cannot be found:** Check that you are running the command from the repository root and that the path is spelled correctly.
- **A typo in `println!` or the braces:** Read the first compiler error and its location. Rust reports where it got confused; fix that issue, then compile again.
- **Trying to run the source file directly:** A `.rs` file is source code. Compile it with `rustc` first, then run the resulting executable.

## Exercises

Try these before looking at the [Day 1 solutions](../../solutions/01-introduction.md).

1. Change the message to print your favorite hobby. Compile and run the program again.
2. Print two separate lines using two `println!` calls. What changes if you use `print!` for one of them instead?
3. Run `rustc --help` and `cargo --help`. Find one option in each that you would like to understand later.
4. In your own words, describe what rustup, rustc, and Cargo each do.

## Further reading

- [Install Rust](https://www.rust-lang.org/tools/install)
- [The Rust Programming Language: Getting Started](https://doc.rust-lang.org/book/ch01-00-getting-started.html)
- [Rust By Example: Hello World](https://doc.rust-lang.org/rust-by-example/hello.html)
- [The Rust compiler (`rustc`) documentation](https://doc.rust-lang.org/rustc/)
- [The Cargo Book](https://doc.rust-lang.org/cargo/)

## Course navigation

- [Course index](../../README.md)
- Previous: none — this is Day 1.
- Next: [Day 2: Variables and Mutability](../02-variables-and-mutability/README.md)
