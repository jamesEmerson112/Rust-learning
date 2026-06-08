#[path = "../src/bin/c58_exercise.rs"]
#[allow(dead_code)]
mod c58_exercise;

use c58_exercise::{MyBox, deref_double};

#[test]
fn deref_doubles() {
    assert_eq!(deref_double(&MyBox::new(21)), 42);
}

#[test]
fn deref_negative() {
    assert_eq!(deref_double(&MyBox::new(-5)), -10);
}
