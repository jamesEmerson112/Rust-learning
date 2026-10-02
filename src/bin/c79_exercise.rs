// BUG: The radiology desk books a scan only if the patient isn't already on the schedule — but
// the moment it goes to add a brand-new booking, the whole program PANICS with a BorrowMutError.
// It's holding the schedule open for reading while another hand tries to write to it. The code
// compiles; it blows up at runtime. Scope the read so the write can happen. Find and fix it.
// (This drills c42-c44/c64: RefCell runtime borrows. The tests in tests/c79_tests.rs must go green.)
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

    pub fn add_if_absent(&self, booking: &str) -> bool {
        let scans = self.scans.borrow();
        if scans.iter().any(|b| b == booking) {
            false
        } else {
            self.add(booking);
            true
        }
    }
}

fn main() {
    let sched = Schedule::new();
    println!("booked Mr. Hung? {}", sched.add_if_absent("Mr. Hung - X-ray"));
    println!("booked Mrs. Lan? {}", sched.add_if_absent("Mrs. Lan - MRI"));
    println!("booked Mr. Hung? {}", sched.add_if_absent("Mr. Hung - X-ray"));
    println!("scans today: {}", sched.len());
}
