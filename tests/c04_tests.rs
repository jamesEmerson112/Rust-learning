#[path = "../src/bin/c04_exercise.rs"]
#[allow(dead_code)]
mod c04_exercise;

use c04_exercise::min_max;

#[test]
fn min_max_positive() {
    let (min, max) = min_max([3, 1, 4, 1, 5]);
    assert_eq!(min, 1);
    assert_eq!(max, 5);
}

#[test]
fn min_max_negative() {
    let (min, max) = min_max([-10, -5, 0, 5, 10]);
    assert_eq!(min, -10);
    assert_eq!(max, 10);
}

#[test]
fn min_max_all_same() {
    let (min, max) = min_max([7, 7, 7, 7, 7]);
    assert_eq!(min, 7);
    assert_eq!(max, 7);
}
