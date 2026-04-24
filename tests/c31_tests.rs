#[path = "../src/bin/c31_exercise.rs"]
#[allow(dead_code)]
mod c31_exercise;

use c31_exercise::share_count;

#[test]
fn three_owners() {
    assert_eq!(share_count(1), 3);
}

#[test]
fn count_independent_of_value() {
    assert_eq!(share_count(999), 3);
}
