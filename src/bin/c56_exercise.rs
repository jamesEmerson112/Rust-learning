// A recursive type: each Client holds the *rest* of the waitlist.
// Without Box this struct would have infinite size — Box stores the rest
// behind a fixed-size heap pointer, so the compiler can size it.
pub enum Waitlist {
    Client(String, Box<Waitlist>),
    Empty,
}

pub fn waitlist_len(list: &Waitlist) -> usize {
    // TODO: Recurse — 1 + the length of the rest, or 0 for Empty.
    let _ = list;
    0
}

pub fn build_waitlist(names: &[&str]) -> Waitlist {
    // TODO: Build a nested Waitlist from the names, ending in Empty.
    // Hint: fold from the back so the first name ends up at the front.
    // (This placeholder just shows the shape — replace it with the full list.)
    let _ = names;
    Waitlist::Client(String::new(), Box::new(Waitlist::Empty))
}

fn main() {
    let list = build_waitlist(&["Mai", "Linh", "Trang"]);
    if let Waitlist::Client(first, _) = &list {
        println!("First in line: {first}");
    }
    println!("Waiting clients: {}", waitlist_len(&list));
}
