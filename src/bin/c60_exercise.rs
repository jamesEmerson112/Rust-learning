// RUST GENERAL HOSPITAL — Safe Shutdown
// When an IV pump session ends, the pump must stop. You'll put that stop in Drop, so it
// runs by itself when the session goes out of scope, on every path out of the code.
use std::cell::RefCell;

pub struct PumpSession<'a> {
    pub bed: String,
    pub log: &'a RefCell<Vec<String>>,
}

impl<'a> Drop for PumpSession<'a> {
    fn drop(&mut self) {
        // TODO: When a PumpSession ends (leaves scope), push
        // "<bed> pump stopped" into the shared log (self.log).
        self.log.borrow_mut().push(format!("{} pump stopped", self.bed))
    }
}

pub fn stop_order() -> Vec<String> {
    let log = RefCell::new(Vec::new());
    {
        // Named variables live until the end of this block.
        // A plain `_` would drop the value immediately, so keep the names.
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
        // TODO: The IV line to bed-1 is blocked, so stop that pump before this
        // scope ends. Pass it to the standard drop() function: drop(bed1)
        drop(bed1);
    }
    log.into_inner()
}

fn main() {
    println!("[pumps] stop_order: {:?}", stop_order());
    println!("[pumps] early_stop: {:?} (bed-1 must stop first)", early_stop());
}
