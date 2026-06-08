#[allow(unused_imports)]
use std::cell::RefCell;

pub fn double_borrow_fails() -> bool {
    // TODO: Hold a borrow_mut, then attempt a SECOND borrow while the first is
    // still alive. RefCell checks borrows at RUNTIME — a plain borrow_mut()
    // would PANIC here, so use try_borrow_mut() and return whether it errored.
    false
}

pub fn borrow_after_drop_ok() -> bool {
    // TODO: Take a borrow_mut inside an inner scope so it is released, then
    // show that a fresh borrow_mut now succeeds.
    false
}

fn main() {
    println!("double borrow fails? {}", double_borrow_fails());
    println!("borrow after drop ok? {}", borrow_after_drop_ok());
}
