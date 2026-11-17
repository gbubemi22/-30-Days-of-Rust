use std::collections::HashMap;
use std::fs;
use std::io;

fn read_config(path: &str) -> io::Result<HashMap<String, String>> {
    let contents = fs::read_to_string(path)?;
    let mut values = HashMap::new();
    for line in contents.lines() {
        if let Some((key, value)) = line.split_once('=') {
            values.insert(key.trim().to_string(), value.trim().to_string());
        }
    }
    Ok(values)
}

fn main() -> io::Result<()> {
    let values = read_config("days/26-environment-and-configuration/examples/app.config.example")?;
    let name = std::env::var("APP_NAME")
        .ok()
        .or_else(|| values.get("APP_NAME").cloned())
        .unwrap_or_else(|| String::from("Application"));
    let mode = std::env::var("APP_MODE")
        .ok()
        .or_else(|| values.get("APP_MODE").cloned())
        .unwrap_or_else(|| String::from("development"));
    let token_configured = std::env::var("APP_TOKEN").is_ok();

    println!("Application: {name}");
    println!("Mode: {mode}");
    println!(
        "Token configured: {}",
        if token_configured { "yes" } else { "no" }
    );
    Ok(())
}
