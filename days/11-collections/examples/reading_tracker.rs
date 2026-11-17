use std::collections::{HashMap, HashSet};

fn main() {
    let mut reading_list = vec!["Rust Book", "Rust By Example"];
    reading_list.push("Standard Library Guide");

    let mut pages_read = HashMap::new();
    pages_read.insert("Rust Book", 35);
    pages_read.insert("Rust By Example", 18);
    pages_read.insert("Standard Library Guide", 4);

    let mut finished = HashSet::new();
    finished.insert("Rust By Example");

    for title in &reading_list {
        let pages = pages_read.get(title).copied().unwrap_or(0);
        let status = if finished.contains(title) {
            "finished"
        } else {
            "in progress"
        };
        println!("{title}: {pages} pages read ({status})");
    }
}
