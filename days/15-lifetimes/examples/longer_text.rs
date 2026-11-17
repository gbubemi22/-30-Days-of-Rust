fn longer<'a>(first: &'a str, second: &'a str) -> &'a str {
    if first.len() >= second.len() {
        first
    } else {
        second
    }
}

fn main() {
    let first = String::from("borrowed text");
    let second = String::from("Rust");
    let result = longer(&first, &second);
    println!("Longer text: {result}");
}
