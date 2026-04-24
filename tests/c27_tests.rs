#[path = "../src/bin/c27_exercise.rs"]
#[allow(dead_code)]
mod c27_exercise;

use c27_exercise::swap;

#[test]
fn swap_integers() {
    assert_eq!(swap(1, 2), (2, 1));
}

#[test]
fn swap_strings() {
    assert_eq!(swap("a", "b"), ("b", "a"));
}

#[test]
fn swap_bools() {
    assert_eq!(swap(true, false), (false, true));
}
