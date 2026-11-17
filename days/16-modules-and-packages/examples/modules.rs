mod catalog;

use catalog::Item;

fn main() {
    let items = catalog::items();
    println!("Available items: {}", items.len());
    let first: &Item = &items[0];
    println!("First item: {}", first.name);
}
