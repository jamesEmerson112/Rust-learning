#[path = "../src/bin/c65_exercise.rs"]
#[allow(dead_code)]
mod c65_exercise;

use c65_exercise::has_duplicate;

#[test]
fn detects_duplicate() {
    assert!(has_duplicate(&[1, 2, 3, 1]));
}

#[test]
fn no_duplicate() {
    assert!(!has_duplicate(&[1, 2, 3, 4]));
}

#[test]
fn empty_has_none() {
    assert!(!has_duplicate(&[]));
}
