#[path = "../src/bin/c59_exercise.rs"]
#[allow(dead_code)]
mod c59_exercise;

use c59_exercise::two_sum;

#[test]
fn finds_pair() {
    assert_eq!(two_sum(&[2, 7, 11, 15], 9), Some((0, 1)));
}

#[test]
fn finds_later_pair() {
    assert_eq!(two_sum(&[3, 2, 4], 6), Some((1, 2)));
}

#[test]
fn no_pair() {
    assert_eq!(two_sum(&[1, 2, 3], 100), None);
}
