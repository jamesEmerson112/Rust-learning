use std::cell::Cell;

struct TipJar {
    total: Cell<u32>,
}

impl TipJar {
    fn new() -> Self {
        Self { total: Cell::new(0) }
    }

    fn add(&self, amount: u32) {
        self.total.set(self.total.get() + amount);
    }

    fn total(&self) -> u32 {
        self.total.get()
    }
}

fn main() {
    let jar = TipJar::new();
    jar.add(500);
    jar.add(300);
    println!("Tips collected: {} cents", jar.total());
}
