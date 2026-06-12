// Interior mutability: Cell<T> lets you mutate a value through a SHARED &reference (normally
// forbidden), for Copy types — get/set/replace/take, no borrows tracked, single-threaded only.
// Coming from C: a small mutable box you can poke even when the struct around it is otherwise
// "const" — but the type system keeps it single-owner, so there are no aliasing surprises.
use std::cell::Cell;

struct Counter {
    value: Cell<u32>,
}

impl Counter {
    fn new(start: u32) -> Self {
        Self { value: Cell::new(start) }
    }
    fn get(&self) -> u32 {
        self.value.get()
    }
    fn replace_with(&self, n: u32) -> u32 {
        self.value.replace(n) // store new, return old
    }
    fn take(&self) -> u32 {
        self.value.take() // return old, leave the default (0)
    }
}

fn main() {
    let c = Counter::new(10);
    println!("old  = {}", c.replace_with(99)); // 10
    println!("now  = {}", c.get());            // 99
    println!("took = {}", c.take());           // 99
    println!("now  = {}", c.get());            // 0
}
