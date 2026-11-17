fn word_count(text: &str) -> usize {
    text.split_whitespace().count()
}

fn add_period(text: &mut String) {
    text.push('.');
}

fn main() {
    let mut note = String::from("Rust makes ownership visible");
    let words = word_count(&note);
    add_period(&mut note);

    println!("{note}");
    println!("Word count: {words}");
}
