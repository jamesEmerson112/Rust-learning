#[path = "../src/bin/c24_exercise.rs"]
#[allow(dead_code)]
mod c24_exercise;

use c24_exercise::product;

#[test]
fn product_basic() {
    assert_eq!(product(&[1, 2, 3, 4]), 24);
}

#[test]
fn product_empty() {
    assert_eq!(product(&[]), 1);
}

#[test]
fn product_with_zero() {
    assert_eq!(product(&[5, 0, 3]), 0);
}
