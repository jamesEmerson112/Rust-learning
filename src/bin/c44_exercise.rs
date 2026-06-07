use std::cell::RefCell;
use std::rc::Rc;

pub fn shared_book_count() -> usize {
    // TODO: Create an Rc<RefCell<Vec<String>>>.
    // Clone it to simulate two stations.
    // Each station pushes one appointment string.
    // Return the total count from the original Rc.
    let book_list = Rc::new(RefCell::new(Vec::new()));

    let station_a = Rc::clone(&book_list);
    let station_b = Rc::clone(&book_list);

    station_a.borrow_mut().push("Mai is taking a pedicure");
    station_b.borrow_mut().push("Jen is taking the gel manicure");

    return book_list.borrow().len()
}

fn main() {
    println!("Shared appointment count: {}", shared_book_count());
}
