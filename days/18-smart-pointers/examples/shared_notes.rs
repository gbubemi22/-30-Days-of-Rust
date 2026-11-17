use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let notes = Rc::new(RefCell::new(vec![String::from("learn ownership")]));
    let first_view = Rc::clone(&notes);
    let second_view = Rc::clone(&notes);

    first_view
        .borrow_mut()
        .push(String::from("practice borrowing"));
    println!("Notes: {:?}", second_view.borrow());
    println!("Owners: {}", Rc::strong_count(&notes));
}
