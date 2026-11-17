pub struct Item {
    pub name: String,
}

pub fn items() -> Vec<Item> {
    vec![
        Item {
            name: String::from("Notebook"),
        },
        Item {
            name: String::from("Pencil"),
        },
    ]
}
