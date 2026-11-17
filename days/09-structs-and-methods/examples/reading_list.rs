struct ReadingItem {
    title: String,
    pages: u32,
    pages_read: u32,
}

impl ReadingItem {
    fn new(title: &str, pages: u32) -> ReadingItem {
        ReadingItem {
            title: String::from(title),
            pages,
            pages_read: 0,
        }
    }

    fn progress(&self) -> u32 {
        self.pages_read * 100 / self.pages
    }

    fn read_pages(&mut self, count: u32) {
        self.pages_read = (self.pages_read + count).min(self.pages);
    }
}

fn main() {
    let mut book = ReadingItem::new("Rust in Small Steps", 100);
    book.read_pages(25);
    println!("{}: {}% complete", book.title, book.progress());
}
