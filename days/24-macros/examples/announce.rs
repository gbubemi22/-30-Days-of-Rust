macro_rules! announce_many {
    ($($message:expr),+ $(,)?) => {
        $(println!("Announcement: {}", $message);)+
    };
}

fn main() {
    announce_many!("Practice", "Read compiler messages", "Keep going");
}
