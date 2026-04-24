use std::rc::Rc;

fn main() {
    let shared = Rc::new(String::from("hello"));
    let a = Rc::clone(&shared);
    let b = Rc::clone(&shared);

    println!("shared = {shared}");
    println!("a      = {a}");
    println!("b      = {b}");
    println!("strong count = {}", Rc::strong_count(&shared));
}
