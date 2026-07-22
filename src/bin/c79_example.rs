// Bug Hunt drill: RefCell borrow scoping. A borrow() guard (Ref) lives until the end of its
// scope and blocks any borrow_mut() while it's alive — a second mutable borrow PANICS at
// runtime (BorrowMutError). Read into a plain value in ONE statement so the Ref is dropped
// before you call a method that needs borrow_mut().
// Coming from C: it's holding a read-lock and then reaching for the write-lock on the same
// mutex without releasing it — except RefCell asserts and aborts instead of deadlocking.
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

    // Adds `name` only if it isn't already booked. Returns true if it was added.
    pub fn add_if_absent(&self, name: &str) -> bool {
        // Read in a single statement so the borrow() guard is dropped right here...
        let already = self.appts.borrow().iter().any(|n| n == name);
        if already {
            false
        } else {
            self.add(name); // ...leaving add()'s borrow_mut() free to run.
            true
        }
    }
}

fn main() {
    let sched = Schedule::new();
    println!("added Mai?   {}", sched.add_if_absent("Mai - Gel Manicure"));
    println!("added Linh?  {}", sched.add_if_absent("Linh - Pedicure"));
    println!("added Mai?   {}", sched.add_if_absent("Mai - Gel Manicure"));
    println!("booked today: {}", sched.len());
}
