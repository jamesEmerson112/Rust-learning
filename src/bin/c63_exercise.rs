// RUST GENERAL HOSPITAL — Shared Care
// The crash-cart defibrillator is shared: every member of the team holds it by
// &shared reference. Its charge still has to change — interior mutability via Cell.
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
        // TODO: Set the new charge and return the PREVIOUS one (the old value) —
        // all through &self. Hint: Cell::replace does exactly this.
        let _ = fresh;
        0
    }

    pub fn shock(&self) -> u32 {
        // TODO: Deliver the ENTIRE charge: return the current value and leave
        // the cell at 0 (u32::default()). Hint: Cell::take.
        0
    }
}

fn main() {
    let defib = Defibrillator::new(150);
    println!("[defib] previous charge: {} J (want 150)", defib.recharge(200));
    println!("[defib] shock: {} J (want 200)", defib.shock());
    println!("[defib] level: {} J (want 0)", defib.charge_level());
}
