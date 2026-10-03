// Interior mutability means changing a value through a shared & reference, which Rust
// normally forbids. Cell<T> provides it for Copy types. You move values in and out with get,
// set, replace, and take, and you never hold a reference to the inside, so there are no
// borrows to track. Cell works only within a single thread.
// Coming from C: think of a small mutable field that you can change even when the struct
// around it is const. The type system still rules out aliasing surprises, because Cell never
// gives out a pointer to its contents.
//
// RUST GENERAL HOSPITAL: the crash-cart defibrillator is shared equipment. Everyone on the
// team holds it by shared reference, but its charge still has to change. Cell makes that
// possible.
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
        self.charge.replace(fresh) // set the new charge and return the previous one
    }
    fn shock(&self) -> u32 {
        self.charge.take() // deliver all of the charge and leave the default value, 0
    }
}

fn main() {
    let defib = Defibrillator::new(150);
    println!("[defib] recharged to 200 J; previous charge was {} J", defib.recharge(200));
    println!("[defib] charge now: {} J", defib.charge_level());
    println!("[defib] SHOCK: {} J delivered", defib.shock());
    println!("[defib] charge now: {} J", defib.charge_level());
}
