#[allow(unused_imports)]
use std::rc::Rc;

pub fn share_count(n: i32) -> usize {
    // TODO: Create an Rc holding `n`, clone it twice, and return the
    // strong count (which should be 3: the original + 2 clones).
    // Hint: let a = Rc::new(n); let _b = Rc::clone(&a); ...
    //       then return Rc::strong_count(&a).
    let _ = n;
    0
}

fn main() {
    println!("share_count(42) = {}", share_count(42));
}
