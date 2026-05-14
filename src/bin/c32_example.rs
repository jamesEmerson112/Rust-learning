#[path = "../lesson32/example_client_counter.rs"]
mod counter;

use counter::ClientCounter;

fn main() {
    let mut c = ClientCounter::new();
    println!("Walk-ins today: {}", c.value());

    c.increment();
    c.increment();
    c.increment();
    println!("After 3 walk-ins: {}", c.value());
}
