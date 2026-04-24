#[path = "../src/bin/c25_exercise.rs"]
#[allow(dead_code)]
mod c25_exercise;

use c25_exercise::debug_string;

#[test]
fn debug_basic() {
    assert_eq!(debug_string(&[1, 2, 3]), "[1, 2, 3]");
}

#[test]
fn debug_empty() {
    assert_eq!(debug_string(&[]), "[]");
}

#[test]
fn debug_single() {
    assert_eq!(debug_string(&[42]), "[42]");
}
