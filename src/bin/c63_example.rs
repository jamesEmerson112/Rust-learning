// Interior mutability: Cell<T> lets you mutate a value through a SHARED &reference (normally
// forbidden), for Copy types — get/set/replace/take, no borrows tracked, single-threaded only.
// Coming from C: a small mutable box you can poke even when the struct around it is otherwise
// "const" — but the type system keeps it single-owner, so there are no aliasing surprises.
//
// THE VAULT RUN: your signal jammer rides in a sealed housing — everyone holds it by
// &shared reference, but the charge cell inside still swaps and drains. That's Cell.
use std::cell::Cell;

struct SignalJammer {
    charge: Cell<u32>,
}

impl SignalJammer {
    fn new(charge: u32) -> Self {
        Self { charge: Cell::new(charge) }
    }
    fn charge_level(&self) -> u32 {
        self.charge.get()
    }
    fn reload(&self, fresh: u32) -> u32 {
        self.charge.replace(fresh) // slot the fresh cell, return the spent one
    }
    fn discharge(&self) -> u32 {
        self.charge.take() // dump ALL charge into the jam, leave the default (0)
    }
}

fn main() {
    let jammer = SignalJammer::new(10);
    println!("[jammer] spent cell pulled: {} units", jammer.reload(99));
    println!("[jammer] charge now: {}", jammer.charge_level());
    println!("[jammer] FULL DISCHARGE: {} units into the jam", jammer.discharge());
    println!("[jammer] charge now: {}", jammer.charge_level());
}
