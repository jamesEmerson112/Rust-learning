#[path = "../src/bin/c07_exercise.rs"]
#[allow(dead_code)]
mod c07_exercise;

use c07_exercise::shout;

#[test]
fn shout_basic() {
    assert_eq!(shout("hello"), "HELLO!");
}

#[test]
fn shout_empty() {
    assert_eq!(shout(""), "!");
}

#[test]
fn shout_mixed_case() {
    assert_eq!(shout("Rust"), "RUST!");
}
