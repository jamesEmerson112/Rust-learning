use std::cell::Cell;

pub struct Counter {
    value: Cell<u32>,
}

impl Counter {
    pub fn new(start: u32) -> Self {
        Self { value: Cell::new(start) }
    }

    pub fn get(&self) -> u32 {
        self.value.get()
    }

    pub fn replace_with(&self, n: u32) -> u32 {
        // TODO: Store `n` and return the OLD value — all through &self.
        // Hint: Cell::replace does exactly this.
        let _ = n;
        0
    }

    pub fn take(&self) -> u32 {
        // TODO: Return the current value and reset the cell to 0 (u32::default()).
        // Hint: Cell::take.
        0
    }
}

fn main() {
    let c = Counter::new(10);
    println!("old  = {}", c.replace_with(99));
    println!("now  = {}", c.get());
    println!("took = {}", c.take());
    println!("now  = {}", c.get());
}
