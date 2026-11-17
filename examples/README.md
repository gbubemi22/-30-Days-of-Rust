# Examples

Most examples live inside their lesson's `days/NN-topic/examples/` directory so the lesson and code stay together. Small examples can be compiled with the `rustc` command shown in that day's lesson.

Examples that need dependencies or Cargo project structure have their own `Cargo.toml` and are members of the root Cargo workspace. Run those examples with the package-specific command in the lesson, or use `cargo run -p PACKAGE_NAME` from the repository root.
