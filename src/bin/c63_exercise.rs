// THE VAULT RUN — Chapter 3: INSIDE THE ICE
// The jammer's housing is sealed: every crew member holds it by &shared reference.
// The charge cell inside still has to swap and drain — interior mutability via Cell.
use std::cell::Cell;

pub struct SignalJammer {
    charge: Cell<u32>,
}

impl SignalJammer {
    pub fn new(charge: u32) -> Self {
        Self { charge: Cell::new(charge) }
    }

    pub fn charge_level(&self) -> u32 {
        self.charge.get()
    }

    pub fn reload(&self, fresh: u32) -> u32 {
        // TODO: Slot the fresh cell and return the SPENT one (the old value) —
        // all through &self. Hint: Cell::replace does exactly this.
        let _ = fresh;
        0
    }

    pub fn discharge(&self) -> u32 {
        // TODO: Dump the ENTIRE charge into the jam: return the current value
        // and leave the cell at 0 (u32::default()). Hint: Cell::take.
        0
    }
}

fn main() {
    let jammer = SignalJammer::new(10);
    println!("[jammer] spent cell: {} (want 10)", jammer.reload(99));
    println!("[jammer] discharge: {} (want 99)", jammer.discharge());
    println!("[jammer] level: {} (want 0)", jammer.charge_level());
}
