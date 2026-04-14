#[path = "../src/bin/c27_exercise.rs"]
#[allow(dead_code)]
mod c27_exercise;

use c27_exercise::boxed_number;

#[test]
fn boxed_holds_value() {
    let b = boxed_number(7);
    assert_eq!(*b, 7);
}

#[test]
fn boxed_handles_negative() {
    let b = boxed_number(-42);
    assert_eq!(*b, -42);
}

#[test]
fn boxed_zero() {
    let b = boxed_number(0);
    assert_eq!(*b, 0);
}
