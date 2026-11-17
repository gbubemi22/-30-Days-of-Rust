fn add_exclamation(mut text: String) -> String {
    text.push('!');
    text
}

fn main() {
    let title = String::from("Learning Rust");
    let updated_title = add_exclamation(title);
    println!("{updated_title}");

    let chapters = 6;
    let completed_chapters = chapters;
    println!("Completed {completed_chapters} of {chapters} chapters");
}
