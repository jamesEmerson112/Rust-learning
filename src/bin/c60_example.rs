// Drop runs cleanup automatically when a value leaves scope, in reverse (LIFO) order — Rust's
// destructor. This is RAII, not garbage collection: it fires at a known, deterministic point.
// Coming from C: it's the free()/close()/unlock() you write by hand at every return and
// `goto fail`, except the compiler guarantees it runs exactly once, on every exit path.
//
// THE VAULT RUN: every uplink you open into Aegis-9 MUST burn its trace on disconnect.
// Forget one and the trace daemon follows it home. Drop means you CAN'T forget.
use std::cell::RefCell;

struct Uplink<'a> {
    handle: String,
    log: &'a RefCell<Vec<String>>,
}

impl<'a> Drop for Uplink<'a> {
    fn drop(&mut self) {
        self.log.borrow_mut().push(format!("{} trace burned", self.handle));
    }
}

fn main() {
    let log = RefCell::new(Vec::new());
    {
        let _alpha = Uplink { handle: "alpha".to_string(), log: &log };
        let _bravo = Uplink { handle: "bravo".to_string(), log: &log };
        println!("[uplink] both channels hot.");
    } // _bravo burns first, then _alpha — reverse of open order
    println!("[uplink] burn log: {:?}", log.borrow());

    let log2 = RefCell::new(Vec::new());
    {
        let alpha = Uplink { handle: "alpha".to_string(), log: &log2 };
        let _bravo = Uplink { handle: "bravo".to_string(), log: &log2 };
        drop(alpha); // burn alpha early, on YOUR schedule — still exactly once
        println!("[uplink] alpha burned early, bravo still hot.");
    }
    println!("[uplink] burn log: {:?}", log2.borrow());
}
