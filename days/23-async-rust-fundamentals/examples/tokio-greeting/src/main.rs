use std::time::Duration;

async fn greet(name: &'static str, delay_ms: u64) -> &'static str {
    tokio::time::sleep(Duration::from_millis(delay_ms)).await;
    name
}

#[tokio::main]
async fn main() {
    println!("Starting two async tasks...");
    let (first, second) = tokio::join!(greet("Ada", 20), greet("Linus", 10));
    println!("Hello, {first}!");
    println!("Hello, {second}!");
    println!("Both tasks finished.");
}
