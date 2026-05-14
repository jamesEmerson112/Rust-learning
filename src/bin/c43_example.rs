use std::cell::RefCell;

struct Schedule {
    appts: RefCell<Vec<String>>,
}

impl Schedule {
    fn new() -> Self {
        Self { appts: RefCell::new(Vec::new()) }
    }

    fn add(&self, name: &str) {
        self.appts.borrow_mut().push(name.to_string());
    }

    fn count(&self) -> usize {
        self.appts.borrow().len()
    }
}

fn main() {
    let sched = Schedule::new();
    sched.add("Mai - Gel Manicure");
    sched.add("Linh - Pedicure");
    println!("Appointments today: {}", sched.count());
}
