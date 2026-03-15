#[path = "../src/bin/c09_exercise.rs"]
#[allow(dead_code)]
mod c09_exercise;

use c09_exercise::{Point, distance_from_origin};

#[test]
fn distance_three_four_is_five() {
    let p = Point { x: 3.0, y: 4.0 };
    assert!((distance_from_origin(&p) - 5.0).abs() < 1e-9);
}

#[test]
fn distance_origin_is_zero() {
    let p = Point { x: 0.0, y: 0.0 };
    assert!((distance_from_origin(&p) - 0.0).abs() < 1e-9);
}
