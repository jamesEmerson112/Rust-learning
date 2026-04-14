#[path = "../src/bin/c22_exercise.rs"]
#[allow(dead_code)]
mod c22_exercise;

use c22_exercise::sum_of_squares;

#[test]
fn basic() {
    // 1 + 4 + 9 + 16 = 30
    assert_eq!(sum_of_squares(&[1, 2, 3, 4]), 30);
}

#[test]
fn handles_zeros_and_negatives() {
    assert_eq!(sum_of_squares(&[-3, 0, 4]), 9 + 0 + 16);
}

#[test]
fn empty_is_zero() {
    assert_eq!(sum_of_squares(&[]), 0);
}
