// Drop runs cleanup automatically when a value leaves scope, in reverse (LIFO) order — Rust's
// destructor. This is RAII, not garbage collection: it fires at a known, deterministic point.
// Coming from C: it's the free()/close()/unlock() you write by hand at every return and
// `goto fail`, except the compiler guarantees it runs exactly once, on every exit path.
use std::cell::RefCell;

struct Station<'a> {
    name: String,
    log: &'a RefCell<Vec<String>>,
}

impl<'a> Drop for Station<'a> {
    fn drop(&mut self) {
        self.log.borrow_mut().push(format!("closing {}", self.name));
    }
}

fn main() {
    let log = RefCell::new(Vec::new());
    {
        let _a = Station { name: "A".to_string(), log: &log };
        let _b = Station { name: "B".to_string(), log: &log };
        println!("Both stations open.");
    } // _b drops first, then _a — reverse of declaration order
    println!("{:?}", log.borrow());
}
