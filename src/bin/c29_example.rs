#[path = "../lesson29/example_counter.rs"]
mod counter;

use counter::Counter;

fn main() {
    let mut c = Counter::new();
    println!("Initial: {}", c.value());

    c.increment();
    c.increment();
    c.increment();
    println!("After 3 increments: {}", c.value());
}
