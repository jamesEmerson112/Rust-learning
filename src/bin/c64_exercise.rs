// THE VAULT RUN — Chapter 3: INSIDE THE ICE — ★ BUG HUNT ★
//
// BUG: The deck PANICS mid-run — "already mutably borrowed" / BorrowMutError.
// The scan daemon writes to the intrusion log while a sweep still holds it open.
// The compiler can't save you here: RefCell checks borrows at RUNTIME.
// Aegis-9's ICE loves a crashed deck. Refuse the contended write gracefully instead.
//
// Find it, fix it: cargo test --test c64_tests
use std::cell::RefCell;

pub struct IntrusionLog {
    pub entries: RefCell<Vec<String>>,
}

impl IntrusionLog {
    pub fn new() -> Self {
        Self { entries: RefCell::new(Vec::new()) }
    }

    pub fn record(&self, entry: &str) {
        self.entries.borrow_mut().push(entry.to_string());
    }

    pub fn entry_count(&self) -> usize {
        self.entries.borrow().len()
    }

    // A write that arrives DURING a sweep must come back Err — never a panic.
    pub fn record_during_sweep(&self) -> Result<usize, String> {
        let sweep = self.entries.borrow(); // the sweep holds the log open
        self.entries.borrow_mut().push("ping during sweep".to_string());
        Ok(sweep.len())
    }

    // After the sweep releases the log, writes flow again.
    pub fn record_after_sweep(&self) -> usize {
        {
            let _sweep = self.entries.borrow();
        } // sweep released here
        self.record("post-sweep ping");
        self.entry_count()
    }
}

fn main() {
    let log = IntrusionLog::new();
    log.record("breach at relay-7");
    println!("[log] write during sweep: {:?} (want Err(..), not a panic)", log.record_during_sweep());
    println!("[log] write after sweep: {} entries (want 2)", log.record_after_sweep());
    println!("══ when the deck survives the sweep, CHAPTER 3: INSIDE THE ICE is complete ══");
}
