#[path = "../lesson30/example_gradebook.rs"]
mod gradebook;

use gradebook::Gradebook;

fn main() {
    let mut gradebook = Gradebook::new();
    gradebook.add_score("Ava", 90);
    gradebook.add_score("Ava", 84);
    gradebook.add_score("Ben", 76);

    if let Some(avg) = gradebook.average("Ava") {
        println!("Ava average: {avg:.2}");
    }

    match gradebook.average("Zoe") {
        Some(avg) => println!("Zoe average: {avg:.2}"),
        None => println!("No scores for Zoe"),
    }
}
