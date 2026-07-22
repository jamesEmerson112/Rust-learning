// RefCell also gives interior mutability, but for non-Copy data, by moving the borrow check from
// compile time to RUNTIME: borrow()/borrow_mut() track active borrows and PANIC on a violation.
// Coming from C: like a runtime assert that catches aliasing a mutable thing — the safety the
// compiler usually proves statically, enforced dynamically instead. try_borrow_mut avoids the panic.
//
// THE VAULT RUN: the intrusion log is shared kit. While a sweep holds it open for reading,
// a write must be REFUSED gracefully — a panicking deck inside the ice is a dead deck.
use std::cell::RefCell;

struct IntrusionLog {
    entries: RefCell<Vec<String>>,
}

impl IntrusionLog {
    fn new() -> Self {
        Self { entries: RefCell::new(Vec::new()) }
    }
    fn record(&self, entry: &str) {
        self.entries.borrow_mut().push(entry.to_string());
    }
    fn entry_count(&self) -> usize {
        self.entries.borrow().len()
    }
}

fn main() {
    let log = IntrusionLog::new();
    log.record("breach at relay-7");

    {
        let sweep = log.entries.borrow(); // a sweep holds the log open
        // A borrow_mut() *while `sweep` is alive* would PANIC at runtime:
        //   log.entries.borrow_mut(); // thread panics: already borrowed
        // try_borrow_mut() returns Err instead of panicking:
        println!("[log] write during sweep ok? {}", log.entries.try_borrow_mut().is_ok()); // false
        println!("[log] sweep sees {} entries", sweep.len());
    } // `sweep` released here

    println!("[log] write after sweep ok? {}", log.entries.try_borrow_mut().is_ok()); // true
    log.record("counter-ice deployed");
    println!("[log] {} entries total", log.entry_count());
}
