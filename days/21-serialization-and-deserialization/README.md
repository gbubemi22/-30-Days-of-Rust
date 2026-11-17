# Day 21: Serialization and Deserialization

Programs often need to save data or exchange it with other tools. **Serialization** converts structured values into a format such as JSON; **deserialization** reads that format back into typed values. This lesson introduces JSON and the Serde ecosystem, which connects Rust types to data formats.

## Learning objectives

- explain serialization and deserialization;
- recognize JSON objects, arrays, strings, numbers, booleans, and `null`;
- use Serde's `Serialize` and `Deserialize` derives;
- convert between a Rust struct and JSON with `serde_json`;
- handle format errors with `Result`.

## Prerequisites

Read [Day 20: File Handling](../20-file-handling/README.md), and review structs and enums. This example uses external crates managed by Cargo.

## JSON and Rust data

JSON represents data with objects (`{}`), arrays (`[]`), strings, numbers, booleans, and `null`. For example:

```json
{
  "title": "Learn Rust",
  "complete": false,
  "minutes": 30
}
```

Serialization turns a Rust value into JSON text. Deserialization parses JSON text and constructs a Rust value. The text must match the Rust type's expected fields and value types.

## Serde and `serde_json`

Serde defines the `Serialize` and `Deserialize` traits. Format crates such as `serde_json` provide JSON support. Derive the traits on your data type, then call `serde_json::to_string_pretty` or `serde_json::from_str`:

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Task {
    title: String,
    complete: bool,
}
```

Cargo.toml needs Serde with its derive feature and the JSON crate. See the provided example manifest for the dependency declarations. For current instructions, consult [Serde's derive guide](https://serde.rs/derive.html) and [`serde_json` documentation](https://docs.rs/serde_json/).

## Practical example

The example is a Cargo project in [`examples/json_tasks/`](examples/json_tasks/). Run it from the repository root:

```sh
cargo run --manifest-path days/21-serialization-and-deserialization/examples/json_tasks/Cargo.toml
```

The first build downloads the declared crates if needed. Expected output:

```text
{
  "title": "Practice Rust",
  "complete": false,
  "minutes": 20
}
Restored task: Task { title: "Practice Rust", complete: false, minutes: 20 }
```

The program serializes a `Task` to pretty JSON and deserializes that JSON back into a `Task`. Both conversions return `Result`, so `main` propagates any serialization or parsing error.

## Common mistakes

- **Forgetting a derive:** A type must implement `Serialize` or `Deserialize` before the corresponding conversion can use it.
- **JSON field does not match the struct:** A missing required field or wrong value type produces a deserialization error.
- **Treating JSON as a Rust type:** JSON is text; deserialization parses that text into a Rust value.
- **Using `unwrap()` on external data:** JSON from a file or user may be malformed. Return or handle the error.
- **Adding crates by editing source alone:** Declare external dependencies in `Cargo.toml` so Cargo can resolve them.

## Exercises

Try these before reading the [Day 21 solutions](../../solutions/21-serialization-and-deserialization.md).

1. Add a `priority: u8` field to `Task` and serialize it.
2. Deserialize a JSON string into a task and print its title.
3. Try malformed JSON and display the error without panicking.
4. Save the serialized JSON to a file and read it back using `std::fs`.
5. Explain the roles of Serde and `serde_json`.

## Further reading

- [Serde: Derive](https://serde.rs/derive.html)
- [`serde_json` documentation](https://docs.rs/serde_json/)
- [Serde data model](https://serde.rs/data-model.html)

## Course navigation

- [Course index](../../README.md)
- Previous: [Day 20: File Handling](../20-file-handling/README.md)
- Next: [Day 22: Concurrency Fundamentals](../22-concurrency-fundamentals/README.md)
