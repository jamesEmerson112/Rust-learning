// Bug Hunt drill: RefCell borrow scoping. A borrow() guard (Ref) lives until the end of its
// scope and blocks any borrow_mut() while it's alive — a second mutable borrow PANICS at
// runtime (BorrowMutError). Read into a plain value in ONE statement so the Ref is dropped
// before you call a method that needs borrow_mut().
// Coming from C: it's holding a read-lock and then reaching for the write-lock on the same
// mutex without releasing it — except RefCell asserts and aborts instead of deadlocking.
use std::cell::RefCell;

pub struct Schedule {
    scans: RefCell<Vec<String>>,
}

impl Schedule {
    pub fn new() -> Self {
        Self { scans: RefCell::new(Vec::new()) }
    }

    pub fn add(&self, booking: &str) {
        self.scans.borrow_mut().push(booking.to_string());
    }

    pub fn len(&self) -> usize {
        self.scans.borrow().len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    // Adds `booking` only if it isn't already on the schedule. Returns true if it was added.
    pub fn add_if_absent(&self, booking: &str) -> bool {
        // Read in a single statement so the borrow() guard is dropped right here...
        let already = self.scans.borrow().iter().any(|b| b == booking);
        if already {
            false
        } else {
            self.add(booking); // ...leaving add()'s borrow_mut() free to run.
            true
        }
    }
}

fn main() {
    let sched = Schedule::new();
    println!("booked Mr. Hung?  {}", sched.add_if_absent("Mr. Hung - X-ray"));
    println!("booked Mrs. Lan?  {}", sched.add_if_absent("Mrs. Lan - MRI"));
    println!("booked Mr. Hung?  {}", sched.add_if_absent("Mr. Hung - X-ray"));
    println!("scans today: {}", sched.len());
}
