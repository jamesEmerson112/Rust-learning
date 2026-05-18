// EXERCISE: Edit src/lesson32/exercise_client_counter.rs
// Complete the ClientCounter methods: increment() and value().
// This file demonstrates `mod` + `#[path]` — it's the "wiring", not the logic.

#[path = "../lesson32/exercise_client_counter.rs"]
mod counter;

pub use counter::ClientCounter;

fn main() {
    let mut c = ClientCounter::new();
    c.increment();
    c.increment();
    println!("Walk-ins: {}", c.value());
}
