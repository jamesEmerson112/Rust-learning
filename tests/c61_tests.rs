#[path = "../src/bin/c61_exercise.rs"]
#[allow(dead_code)]
mod c61_exercise;

use c61_exercise::{dangling_weak_is_none, weak_counts};

#[test]
fn one_strong_one_weak() {
    assert_eq!(weak_counts(), (1, 1));
}

#[test]
fn weak_cannot_outlive_owner() {
    assert!(dangling_weak_is_none());
}
