// RUST GENERAL HOSPITAL — Safe Shutdown
// Rule one for infusion pumps: when a pump session ends, the pump stops, automatically.
// That's Drop — RAII cleanup at a deterministic point, LIFO order, every exit path.
use std::cell::RefCell;

pub struct PumpSession<'a> {
    pub bed: String,
    pub log: &'a RefCell<Vec<String>>,
}

impl<'a> Drop for PumpSession<'a> {
    fn drop(&mut self) {
        // TODO: When a PumpSession ends (leaves scope), push
        // "<bed> pump stopped" into the shared log (self.log).
        let _ = (&self.bed, &self.log);
    }
}

pub fn stop_order() -> Vec<String> {
    let log = RefCell::new(Vec::new());
    {
        // Named bindings live until the end of this block.
        // A bare `_` would drop *immediately* — keep the names.
        let _bed1 = PumpSession { bed: "bed-1".to_string(), log: &log };
        let _bed2 = PumpSession { bed: "bed-2".to_string(), log: &log };
    } // stopped here in reverse order: bed-2 first, then bed-1
    log.into_inner()
}

pub fn early_stop() -> Vec<String> {
    let log = RefCell::new(Vec::new());
    {
        let bed1 = PumpSession { bed: "bed-1".to_string(), log: &log };
        let _bed2 = PumpSession { bed: "bed-2".to_string(), log: &log };
        // TODO: The line to bed-1 is blocked — stop that pump FIRST, before
        // this scope ends. Hand it to std's drop(): drop(bed1)
    }
    log.into_inner()
}

fn main() {
    println!("[pumps] stop_order: {:?}", stop_order());
    println!("[pumps] early_stop: {:?} (bed-1 must stop first)", early_stop());
}
