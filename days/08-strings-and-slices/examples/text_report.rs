fn first_word(text: &str) -> &str {
    text.split_whitespace().next().unwrap_or("")
}

fn main() {
    let mut note = String::from("Rust makes careful programs");
    let greeting = format!("Study note: {note}");
    let word = first_word(&note);
    let scores = [8, 9, 10, 7];
    let recent_scores = &scores[1..3];

    println!("{greeting}");
    println!("First word: {word}");
    println!("Recent scores: {recent_scores:?}");

    note.push('!');
    println!("Updated note: {note}");
}
