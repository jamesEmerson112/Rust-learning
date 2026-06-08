use std::ops::Deref;

pub struct MyBox<T>(T);

impl<T> MyBox<T> {
    pub fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        // TODO: Return a reference to the inner value (self.0).
        // This is what makes `*my_box` work and enables auto-deref coercion.
        todo!()
    }
}

pub fn deref_double(b: &MyBox<i32>) -> i32 {
    // TODO: Dereference twice (&MyBox<i32> -> MyBox<i32> -> i32) and double it.
    // Hint: **b * 2
    let _ = b;
    0
}

fn main() {
    let b = MyBox::new(21);
    println!("deref_double = {}", deref_double(&b));
}
