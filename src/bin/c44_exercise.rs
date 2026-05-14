use std::cell::RefCell;
use std::rc::Rc;

pub fn shared_book_count() -> usize {
    // TODO: Create an Rc<RefCell<Vec<String>>>.
    // Clone it to simulate two stations.
    // Each station pushes one appointment string.
    // Return the total count from the original Rc.
    0
}

fn main() {
    println!("Shared appointment count: {}", shared_book_count());
}
