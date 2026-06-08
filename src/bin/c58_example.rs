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
