// THE VAULT RUN — Chapter 2: GHOST PROTOCOL
// Rule one of the run: every uplink burns its trace on disconnect, automatically.
// That's Drop — RAII cleanup at a deterministic point, LIFO order, every exit path.
use std::cell::RefCell;

pub struct Uplink<'a> {
    pub handle: String,
    pub log: &'a RefCell<Vec<String>>,
}

impl<'a> Drop for Uplink<'a> {
    fn drop(&mut self) {
        // TODO: When an Uplink disconnects (leaves scope), push
        // "<handle> trace burned" into the shared log (self.log).
        let _ = (&self.handle, &self.log);
    }
}

pub fn burn_order() -> Vec<String> {
    let log = RefCell::new(Vec::new());
    {
        // Named bindings live until the end of this block.
        // A bare `_` would drop *immediately* — keep the names.
        let _alpha = Uplink { handle: "alpha".to_string(), log: &log };
        let _bravo = Uplink { handle: "bravo".to_string(), log: &log };
    } // disconnected here in reverse order: bravo first, then alpha
    log.into_inner()
}

pub fn early_burn() -> Vec<String> {
    let log = RefCell::new(Vec::new());
    {
        let alpha = Uplink { handle: "alpha".to_string(), log: &log };
        let _bravo = Uplink { handle: "bravo".to_string(), log: &log };
        // TODO: The lookout says alpha is compromised — burn it FIRST, before
        // this scope ends. Hand it to std's drop(): drop(alpha)
    }
    log.into_inner()
}

fn main() {
    println!("[uplink] burn_order: {:?}", burn_order());
    println!("[uplink] early_burn: {:?} (alpha must burn first)", early_burn());
}
