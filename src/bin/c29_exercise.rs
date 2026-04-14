#[path = "../lesson29/exercise_counter.rs"]
mod counter;

pub use counter::Counter;

fn main() {
    let mut c = Counter::new();
    c.increment();
    c.increment();
    println!("Count: {}", c.value());
}
