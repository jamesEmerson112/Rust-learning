#[path = "../src/bin/c59_exercise.rs"]
#[allow(dead_code)]
mod c59_exercise;

use c59_exercise::trip_pair;

#[test]
fn finds_the_pair() {
    assert_eq!(trip_pair(&[2, 7, 11, 15], 9), Some((0, 1)));
}

#[test]
fn finds_a_later_pair() {
    assert_eq!(trip_pair(&[3, 2, 4], 6), Some((1, 2)));
}

#[test]
fn no_pair_fits_the_charge() {
    assert_eq!(trip_pair(&[1, 2, 3], 100), None);
}
