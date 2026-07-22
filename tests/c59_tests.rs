#[path = "../src/bin/c59_exercise.rs"]
#[allow(dead_code)]
mod c59_exercise;

use c59_exercise::master_key_pair;

#[test]
fn finds_the_key_pair() {
    assert_eq!(master_key_pair(&[2, 7, 11, 15], 9), Some((0, 1)));
}

#[test]
fn finds_a_later_pair() {
    assert_eq!(master_key_pair(&[3, 2, 4], 6), Some((1, 2)));
}

#[test]
fn garbage_burst_has_no_pair() {
    assert_eq!(master_key_pair(&[1, 2, 3], 100), None);
}
