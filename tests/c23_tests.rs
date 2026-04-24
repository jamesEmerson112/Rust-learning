#[path = "../src/bin/c23_exercise.rs"]
#[allow(dead_code)]
mod c23_exercise;

use c23_exercise::total;

#[test]
fn total_basic() {
    assert_eq!(total(&[1, 2, 3, 4]), 10);
}

#[test]
fn total_empty() {
    assert_eq!(total(&[]), 0);
}

#[test]
fn total_negatives() {
    assert_eq!(total(&[-1, 2, -3]), -2);
}
