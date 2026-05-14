use std::cell::Cell;

pub struct TipJar {
    total: Cell<u32>,
}

impl TipJar {
    pub fn new() -> Self {
        Self { total: Cell::new(0) }
    }

    pub fn add(&self, amount: u32) {
        // TODO: Add amount to total using Cell::get and Cell::set.
        // Note: &self, not &mut self!
        let _ = amount;
    }

    pub fn total(&self) -> u32 {
        // TODO: Return the current total.
        0
    }
}

fn main() {
    let jar = TipJar::new();
    jar.add(500);
    println!("Tips: {} cents", jar.total());
}
