// BUG: The radiology desk books a scan only if the patient is not already on the schedule.
// But as soon as it tries to add a new booking, the whole program panics with a
// BorrowMutError. The schedule is still open for reading when the code tries to write to it.
// The code compiles, and the failure happens only at run time. Find the bug and fix it.
// This drills RefCell runtime borrows from c42 to c44 and c64. The tests in
// tests/c79_tests.rs must pass.
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
