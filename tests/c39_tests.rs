#[path = "../src/bin/c39_exercise.rs"]
#[allow(dead_code)]
mod c39_exercise;

use c39_exercise::daily_revenue;

#[test]
fn sums_prices() {
    assert_eq!(daily_revenue(&[4500, 3500, 6000]), 14000);
}

#[test]
fn empty_slice() {
    assert_eq!(daily_revenue(&[]), 0);
}

#[test]
fn single_service() {
    assert_eq!(daily_revenue(&[2500]), 2500);
}
