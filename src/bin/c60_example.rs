// Drop is Rust's destructor. When a value goes out of scope, Rust calls its drop() method
// automatically. Values in the same scope are dropped in reverse order, so the last one
// created is the first one cleaned up. This happens at a fixed point in the code, not later
// whenever a garbage collector runs.
// Coming from C: think of the free(), close(), or unlock() calls you write before every
// return. With Drop, the compiler inserts that call for you on every way out of the scope,
// and it runs exactly once.
//
// RUST GENERAL HOSPITAL: when an IV pump session ends, the pump has to stop. That must
// happen whether the code finishes normally, returns early, or panics. Putting the stop
// in Drop means no code path can skip it.
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
    } // _bed2 stops first, then _bed1, the reverse of the order they started
    println!("[pumps] stop log: {:?}", log.borrow());

    let log2 = RefCell::new(Vec::new());
    {
        let bed1 = PumpSession { bed: "bed-1".to_string(), log: &log2 };
        let _bed2 = PumpSession { bed: "bed-2".to_string(), log: &log2 };
        drop(bed1); // stop bed-1 now instead of at the end of the scope; it still stops only once
        println!("[pumps] bed-1 stopped early, bed-2 still running.");
    }
    println!("[pumps] stop log: {:?}", log2.borrow());
}
