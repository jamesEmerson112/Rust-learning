#[path = "../src/bin/c20_exercise.rs"]
#[allow(dead_code)]
mod c20_exercise;

use c20_exercise::apply;

#[test]
fn apply_double() {
    assert_eq!(apply(5, |n| n * 2), 10);
}

#[test]
fn apply_add_constant() {
    let add_one = |n| n + 1;
    assert_eq!(apply(41, add_one), 42);
}

#[test]
fn apply_captures_from_scope() {
    let factor = 3;
    assert_eq!(apply(5, |n| n * factor), 15);
}
