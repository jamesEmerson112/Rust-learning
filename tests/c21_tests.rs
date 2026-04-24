#[path = "../src/bin/c21_exercise.rs"]
#[allow(dead_code)]
mod c21_exercise;

use c21_exercise::doubled;

#[test]
fn doubled_basic() {
    assert_eq!(doubled(&[1, 2, 3]), vec![2, 4, 6]);
}

#[test]
fn doubled_empty() {
    assert_eq!(doubled(&[]), Vec::<i32>::new());
}

#[test]
fn doubled_negatives() {
    assert_eq!(doubled(&[-1, -2, 0]), vec![-2, -4, 0]);
}
