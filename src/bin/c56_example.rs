enum Waitlist {
    Client(String, Box<Waitlist>),
    Empty,
}

fn waitlist_len(list: &Waitlist) -> usize {
    match list {
        Waitlist::Client(_, rest) => 1 + waitlist_len(rest),
        Waitlist::Empty => 0,
    }
}

fn main() {
    let list = Waitlist::Client(
        "Mai".to_string(),
        Box::new(Waitlist::Client("Linh".to_string(), Box::new(Waitlist::Empty))),
    );
    if let Waitlist::Client(first, _) = &list {
        println!("First in line: {first}");
    }
    println!("Waiting clients: {}", waitlist_len(&list));
}
