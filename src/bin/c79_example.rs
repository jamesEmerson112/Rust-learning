// Bug Hunt drill: how long a RefCell borrow lasts. borrow() returns a guard of type Ref, and
// the guard lives until the end of its scope. While it is alive, any borrow_mut() panics at
// run time with a BorrowMutError. Read what you need into a plain value in a single
// statement, so the Ref is dropped before you call a method that needs borrow_mut().
// Coming from C: this is like holding a read lock and then asking for the write lock on the
// same mutex without releasing the read lock first. RefCell panics in that case, like a
// failed assert, instead of deadlocking.
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
        // Read in one statement, so the borrow() guard is dropped at the end of this line.
        let already = self.scans.borrow().iter().any(|b| b == booking);
        if already {
            false
        } else {
            self.add(booking); // add() can now call borrow_mut() without a panic
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
