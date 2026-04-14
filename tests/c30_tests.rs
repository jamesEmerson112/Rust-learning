#[path = "../src/bin/c30_exercise.rs"]
#[allow(dead_code)]
mod c30_exercise;

use c30_exercise::Gradebook;

#[test]
fn average_for_known_student() {
    let mut gradebook = Gradebook::new();
    gradebook.add_score("Sam", 80);
    gradebook.add_score("Sam", 100);

    let avg = gradebook.average("Sam").expect("expected average for Sam");
    assert!((avg - 90.0).abs() < 1e-9);
}

#[test]
fn average_for_unknown_student_is_none() {
    let gradebook = Gradebook::new();
    assert_eq!(gradebook.average("Unknown"), None);
}

#[test]
fn average_precision_to_two_decimals() {
    let mut gradebook = Gradebook::new();
    gradebook.add_score("Ava", 89);
    gradebook.add_score("Ava", 90);

    let avg = gradebook.average("Ava").expect("expected average for Ava");
    let rounded = (avg * 100.0).round() / 100.0;
    assert_eq!(rounded, 89.5);
}
