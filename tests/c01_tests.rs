#[path = "../src/bin/c01_exercise.rs"]
#[allow(dead_code)]
mod c01_exercise;

use c01_exercise::double;

#[test]
fn double_positive() {
    assert_eq!(double(5), 10);
}

#[test]
fn double_negative() {
    assert_eq!(double(-3), -6);
}
