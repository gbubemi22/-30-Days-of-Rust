use unicode_segmentation::UnicodeSegmentation;

fn main() {
    let text = "hi 👋";
    println!("bytes: {}", text.len());
    println!("graphemes: {}", text.graphemes(true).count());
}
