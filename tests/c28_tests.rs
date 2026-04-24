#[path = "../src/bin/c28_exercise.rs"]
#[allow(dead_code)]
mod c28_exercise;

use c28_exercise::larger;

#[test]
fn larger_ints() {
    assert_eq!(larger(3, 7), 7);
}

#[test]
fn larger_strs() {
    assert_eq!(larger("apple", "banana"), "banana");
}

#[test]
fn larger_equal() {
    assert_eq!(larger(5, 5), 5);
}
