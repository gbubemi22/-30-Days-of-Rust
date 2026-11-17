use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let completed = Arc::new(Mutex::new(Vec::new()));
    let mut handles = Vec::new();

    for worker in vec!["Ada", "Linus"] {
        let completed = Arc::clone(&completed);
        handles.push(thread::spawn(move || {
            let mut names = completed.lock().expect("lock poisoned");
            names.push(worker);
        }));
    }

    for handle in handles {
        handle.join().expect("worker panicked");
    }

    let mut names = completed.lock().expect("lock poisoned");
    names.sort();
    println!("Completed workers: {names:?}");
}
