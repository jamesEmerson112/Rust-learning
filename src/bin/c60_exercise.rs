use std::cell::RefCell;

struct Station<'a> {
    name: String,
    log: &'a RefCell<Vec<String>>,
}

impl<'a> Drop for Station<'a> {
    fn drop(&mut self) {
        // TODO: When a Station goes out of scope, push "closing <name>" into
        // the shared log (self.log). This is RAII — cleanup runs automatically.
        let _ = (&self.name, &self.log);
    }
}

pub fn closing_order() -> Vec<String> {
    let log = RefCell::new(Vec::new());
    {
        // Named bindings (_a, _b) live until the end of this block.
        // A bare `_` would drop *immediately* — keep the names.
        let _a = Station { name: "A".to_string(), log: &log };
        let _b = Station { name: "B".to_string(), log: &log };
    } // dropped here in reverse order: B first, then A
    log.into_inner()
}

fn main() {
    println!("{:?}", closing_order());
}
