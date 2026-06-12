// A type can't contain itself by value — that would be infinitely large. Box<T> is a pointer
// to the heap with a known, fixed size, which is what makes recursive types (lists, trees) work.
// Coming from C: it's exactly why a linked-list node holds a `next` POINTER, not the next node
// inline. Box is that pointer — but it owns what it points to and frees it automatically.
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
