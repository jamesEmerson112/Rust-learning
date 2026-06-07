use std::cell::RefCell;

pub struct Schedule {
    appts: RefCell<Vec<String>>,
}

impl Schedule {
    pub fn new() -> Self {
        Self { appts: RefCell::new(Vec::new()) }
    }

    pub fn add(&self, name: &str) {
        // TODO: Push name into the Vec using borrow_mut().
        // Note: &self, not &mut self!
        self.appts.borrow_mut().push(name.to_string());
    }

    pub fn count(&self) -> usize {
        // TODO: Return the number of appointments using borrow().
        self.appts.borrow().len()
    }
}

fn main() {
    let sched = Schedule::new();
    sched.add("Trang - Dip Powder");
    println!("Appointments: {}", sched.count());
}
