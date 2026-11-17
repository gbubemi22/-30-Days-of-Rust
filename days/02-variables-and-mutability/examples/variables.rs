const MINUTES_PER_HOUR: u32 = 60;

fn main() {
    let event = "Rust study";
    let mut minutes = 25;
    minutes += 5;

    let minutes = minutes.to_string();
    let message = format!("{event} lasts {minutes} minutes.");

    println!("{message}");
    println!("There are {MINUTES_PER_HOUR} minutes in an hour.");
}
