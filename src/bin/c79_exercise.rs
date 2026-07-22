// BUG: The front desk tries to book a walk-in only if she isn't already on the schedule — but
// the moment it goes to add a brand-new name, the whole program PANICS with a BorrowMutError.
// It's holding the schedule open for reading while another hand tries to write to it. The code
// compiles; it blows up at runtime. Scope the read so the write can happen. Find and fix it.
// (This drills c42-c44/c64: RefCell runtime borrows. The tests in tests/c79_tests.rs must go green.)
use std::cell::RefCell;

pub struct Schedule {
    appts: RefCell<Vec<String>>,
}

impl Schedule {
    pub fn new() -> Self {
        Self { appts: RefCell::new(Vec::new()) }
    }

    pub fn add(&self, name: &str) {
        self.appts.borrow_mut().push(name.to_string());
    }

    pub fn len(&self) -> usize {
        self.appts.borrow().len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn add_if_absent(&self, name: &str) -> bool {
        let appts = self.appts.borrow();
        if appts.iter().any(|n| n == name) {
            false
        } else {
            self.add(name);
            true
        }
    }
}

fn main() {
    let sched = Schedule::new();
    println!("added Mai?  {}", sched.add_if_absent("Mai - Gel Manicure"));
    println!("added Linh? {}", sched.add_if_absent("Linh - Pedicure"));
    println!("added Mai?  {}", sched.add_if_absent("Mai - Gel Manicure"));
    println!("booked today: {}", sched.len());
}
