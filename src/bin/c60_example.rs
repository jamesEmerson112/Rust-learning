// Drop runs cleanup automatically when a value leaves scope, in reverse (LIFO) order — Rust's
// destructor. This is RAII, not garbage collection: it fires at a known, deterministic point.
// Coming from C: it's the free()/close()/unlock() you write by hand at every return and
// `goto fail`, except the compiler guarantees it runs exactly once, on every exit path.
//
// RUST GENERAL HOSPITAL: when an IV pump session ends, the pump MUST stop — on the normal
// path, on an early return, even on a panic. Drop means nobody can forget.
use std::cell::RefCell;

struct PumpSession<'a> {
    bed: String,
    log: &'a RefCell<Vec<String>>,
}

impl<'a> Drop for PumpSession<'a> {
    fn drop(&mut self) {
        self.log.borrow_mut().push(format!("{} pump stopped", self.bed));
    }
}

fn main() {
    let log = RefCell::new(Vec::new());
    {
        let _bed1 = PumpSession { bed: "bed-1".to_string(), log: &log };
        let _bed2 = PumpSession { bed: "bed-2".to_string(), log: &log };
        println!("[pumps] both running.");
    } // _bed2 stops first, then _bed1 — reverse of start order
    println!("[pumps] stop log: {:?}", log.borrow());

    let log2 = RefCell::new(Vec::new());
    {
        let bed1 = PumpSession { bed: "bed-1".to_string(), log: &log2 };
        let _bed2 = PumpSession { bed: "bed-2".to_string(), log: &log2 };
        drop(bed1); // stop bed-1 early, on YOUR schedule — still exactly once
        println!("[pumps] bed-1 stopped early, bed-2 still running.");
    }
    println!("[pumps] stop log: {:?}", log2.borrow());
}
