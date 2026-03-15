#[path = "../lesson20/exercise_gradebook.rs"]
mod gradebook;

pub use gradebook::Gradebook;

fn main() {
    let mut gradebook = Gradebook::new();
    gradebook.add_score("Sam", 80);
    gradebook.add_score("Sam", 100);

    println!("Sam average: {:?}", gradebook.average("Sam"));
}
