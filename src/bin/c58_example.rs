// Implementing Deref lets your own wrapper act like a pointer: *mybox works, and method calls
// "auto-deref" through it. This is the trait that makes Box/Rc/Arc feel built into the language.
// Coming from C: it's defining what unary `*` means for your type — a user-overloadable pointer
// dereference, with the compiler inserting the derefs for you where needed.
use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

fn main() {
    let b = MyBox::new(21);
    println!("{}", *b * 2); // *b runs our Deref impl

    let name = MyBox::new(String::from("mai"));
    println!("{}", name.to_uppercase()); // auto-deref reaches String/str methods
}
