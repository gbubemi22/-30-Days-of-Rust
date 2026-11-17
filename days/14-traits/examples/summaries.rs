trait Summary {
    fn summary(&self) -> String;
}
struct Article {
    title: String,
    author: String,
}
struct Note {
    text: String,
}

impl Summary for Article {
    fn summary(&self) -> String {
        format!("{} by {}", self.title, self.author)
    }
}
impl Summary for Note {
    fn summary(&self) -> String {
        format!("Note: {}", self.text)
    }
}
fn show<T: Summary>(item: &T) {
    println!("{}", item.summary());
}

fn main() {
    let article = Article {
        title: String::from("Learning Rust"),
        author: String::from("A learner"),
    };
    let note = Note {
        text: String::from("Practice a little each day"),
    };
    show(&article);
    show(&note);
}
