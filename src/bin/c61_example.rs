use std::rc::{Rc, Weak};

fn main() {
    let station = Rc::new("Station 1".to_string());
    let watcher: Weak<String> = Rc::downgrade(&station);

    // Downgrading does NOT bump the strong count — only the weak count.
    println!(
        "strong = {}, weak = {}",
        Rc::strong_count(&station),
        Rc::weak_count(&station)
    );

    match watcher.upgrade() {
        Some(s) => println!("still alive: {s}"),
        None => println!("gone"),
    }

    drop(station); // last strong owner gone
    println!("after drop, upgrade = {:?}", watcher.upgrade());
}
