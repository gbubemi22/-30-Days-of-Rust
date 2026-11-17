use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Task {
    title: String,
    complete: bool,
    minutes: u32,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let task = Task {
        title: String::from("Practice Rust"),
        complete: false,
        minutes: 20,
    };

    let json = serde_json::to_string_pretty(&task)?;
    println!("{json}");

    let restored: Task = serde_json::from_str(&json)?;
    println!("Restored task: {restored:?}");
    Ok(())
}
