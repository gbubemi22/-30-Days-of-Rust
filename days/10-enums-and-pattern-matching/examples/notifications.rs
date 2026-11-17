enum Notification {
    FriendRequest(String),
    NewMessage { from: String, unread: u32 },
    Reminder,
}

fn describe(notification: Notification) -> String {
    match notification {
        Notification::FriendRequest(name) => format!("{name} sent a friend request"),
        Notification::NewMessage { from, unread } => {
            format!("{from} sent {unread} unread message(s)")
        }
        Notification::Reminder => String::from("You have a reminder"),
    }
}

fn main() {
    println!(
        "{}",
        describe(Notification::FriendRequest(String::from("Ayo")))
    );
    println!(
        "{}",
        describe(Notification::NewMessage {
            from: String::from("Mina"),
            unread: 2,
        })
    );
    println!("{}", describe(Notification::Reminder));
}
