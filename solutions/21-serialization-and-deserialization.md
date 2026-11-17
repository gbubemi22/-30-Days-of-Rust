# Day 21: Serialization and Deserialization — solutions

Try the [exercises](../days/21-serialization-and-deserialization/README.md#exercises) first.

## 1. Add a priority field

```rust
#[derive(Debug, Serialize, Deserialize)]
struct Task {
    title: String,
    complete: bool,
    minutes: u32,
    priority: u8,
}
```

Initialize `priority` when creating a `Task`; the derived serializer includes the field.

## 2. Deserialize and print a title

```rust
let json = r#"{"title":"Read","complete":true,"minutes":10}"#;
let task: Task = serde_json::from_str(json)?;
println!("{}", task.title);
```

## 3. Report invalid JSON

```rust
match serde_json::from_str::<Task>("not JSON") {
    Ok(task) => println!("{}", task.title),
    Err(error) => eprintln!("Invalid task JSON: {error}"),
}
```

## 4. Write and read JSON

```rust
let json = serde_json::to_string(&task)?;
std::fs::write("task.json", &json)?;
let saved = std::fs::read_to_string("task.json")?;
let restored: Task = serde_json::from_str(&saved)?;
```

## 5. Crate roles

Serde defines the common serialization and deserialization traits and derives. `serde_json` implements those traits for JSON text and provides JSON conversion functions.
