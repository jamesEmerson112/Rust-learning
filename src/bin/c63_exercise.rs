// RUST GENERAL HOSPITAL — Shared Care
// The crash-cart defibrillator is shared, and every member of the team holds it by shared
// reference. Its charge still has to change, so you will use Cell for interior mutability.
use std::cell::Cell;

pub struct Defibrillator {
    charge: Cell<u32>, // joules
}

impl Defibrillator {
    pub fn new(charge: u32) -> Self {
        Self { charge: Cell::new(charge) }
    }

    pub fn charge_level(&self) -> u32 {
        self.charge.get()
    }

    pub fn recharge(&self, fresh: u32) -> u32 {
        // TODO: Set the new charge and return the previous one, all through &self.
        // Hint: Cell::replace does exactly this.
        let _ = fresh;
        0
    }

    pub fn shock(&self) -> u32 {
        // TODO: Deliver the entire charge. Return the current value and leave the
        // cell at 0, which is u32::default(). Hint: use Cell::take.
        0
    }
}

fn main() {
    let defib = Defibrillator::new(150);
    println!("[defib] previous charge: {} J (want 150)", defib.recharge(200));
    println!("[defib] shock: {} J (want 200)", defib.shock());
    println!("[defib] level: {} J (want 0)", defib.charge_level());
}
