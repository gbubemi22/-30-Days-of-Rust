struct Pair<T> {
    first: T,
    second: T,
}

impl<T> Pair<T> {
    fn new(first: T, second: T) -> Pair<T> {
        Pair { first, second }
    }
}

fn main() {
    let numbers = Pair::new(12, 30);
    let labels = Pair::new("beginner", "Rust");
    println!("{} and {}", numbers.first, numbers.second);
    println!("{} and {}", labels.first, labels.second);
}
