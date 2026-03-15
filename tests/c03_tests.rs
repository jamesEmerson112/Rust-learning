#[path = "../src/bin/c03_exercise.rs"]
#[allow(dead_code)]
mod c03_exercise;

use c03_exercise::sum_array;

#[test]
fn sum_positive_numbers() {
    assert_eq!(sum_array([1, 2, 3, 4]), 10);
}

#[test]
fn sum_mixed_numbers() {
    assert_eq!(sum_array([-5, 5, -10, 10]), 0);
}
