// Interior mutability: Cell<T> lets you mutate a value through a SHARED &reference (normally
// forbidden), for Copy types — get/set/replace/take, no borrows tracked, single-threaded only.
// Coming from C: a small mutable box you can poke even when the struct around it is otherwise
// "const" — but the type system keeps it single-owner, so there are no aliasing surprises.
//
// RUST GENERAL HOSPITAL: the crash-cart defibrillator is shared kit. Everyone on the team
// holds it by &shared reference, but its charge still has to change. That's Cell.
use std::cell::Cell;

struct Defibrillator {
    charge: Cell<u32>, // joules
}

impl Defibrillator {
    fn new(charge: u32) -> Self {
        Self { charge: Cell::new(charge) }
    }
    fn charge_level(&self) -> u32 {
        self.charge.get()
    }
    fn recharge(&self, fresh: u32) -> u32 {
        self.charge.replace(fresh) // set the new charge, return the previous one
    }
    fn shock(&self) -> u32 {
        self.charge.take() // deliver ALL the charge, leave the default (0)
    }
}

fn main() {
    let defib = Defibrillator::new(150);
    println!("[defib] recharged to 200 J; previous charge was {} J", defib.recharge(200));
    println!("[defib] charge now: {} J", defib.charge_level());
    println!("[defib] SHOCK: {} J delivered", defib.shock());
    println!("[defib] charge now: {} J", defib.charge_level());
}
